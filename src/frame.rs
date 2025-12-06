use crate::config::Args;
use clap::Parser;
use log::{error, info};
use prettytable::{Table, row};

/// WebSocket frame builder
/// allows to configure the frame
///
pub(crate) struct FrameBuilder {
    fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    opcode: u8,
    mask: bool,
    len: usize,
    extended_length_if_126_or_127: u64,
    masking_key: Option<[u8; 4]>,
    payload: Vec<u8>,
}

impl FrameBuilder {
    /// Parse the masking key, even with wrong input
    /// which is required for the exploit
    ///
    fn parse_masking_key(args: &Args) -> Option<[u8; 4]> {
        let key: Result<[u8; 4], _> = args.frame.key.as_slice().try_into();

        if args.bits.mask {
            match key {
                Ok(key) => Some(key),
                Err(e) => {
                    error!("Failed to parse masking key: {}", e);

                    None
                }
            }
        } else {
            None
        }
    }

    /// Parse the payload, even with wrong input
    /// which is required for the exploit
    ///
    fn parse_payload(args: &Args) -> Vec<u8> {
        if args.frame.payload.is_empty() && args.frame.payload[0] == 0 {
            Vec::new()
        } else {
            args.frame.payload.clone()
        }
    }

    pub(crate) fn new() -> Self {
        let args = Args::parse();

        Self {
            fin: args.bits.fin,
            rsv1: args.bits.rsv1,
            rsv2: args.bits.rsv2,
            rsv3: args.bits.rsv3,
            opcode: args.frame.opcode,
            mask: args.bits.mask,
            len: args.frame.len,
            extended_length_if_126_or_127: args.frame.extended_length,
            masking_key: Self::parse_masking_key(&args),
            payload: Self::parse_payload(&args),
        }
    }

    /// Adds extended length to the frame
    ///
    /// If the length is 126, a U16 is added,
    /// if the length is 127, a U64 is added.
    ///
    fn add_extended_length(value: &FrameBuilder, frame: &mut Vec<u8>) {
        if value.len == 126 {
            frame.push((value.extended_length_if_126_or_127 >> 8) as u8);
            frame.push(value.extended_length_if_126_or_127 as u8);
        } else if value.len == 127 {
            frame.push((value.extended_length_if_126_or_127 >> 56) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 48) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 40) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 32) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 24) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 16) as u8);
            frame.push((value.extended_length_if_126_or_127 >> 8) as u8);
            frame.push(value.extended_length_if_126_or_127 as u8);
        }
    }

    /// Adds payload to the frame
    ///
    /// If the payload is masked, a masking key is added.
    ///
    fn add_payload(value: &FrameBuilder, frame: &mut Vec<u8>) {
        if value.mask && value.payload.len() >= 4 {
            let masking_key = match value.masking_key {
                Some(key) => key,
                None => {
                    error!("Masking key not set in FrameBuilder config!");

                    unreachable!()
                }
            };

            frame.extend_from_slice(&masking_key);

            let mut masked_payload = Vec::with_capacity(value.payload.len());

            for (i, byte) in value.payload.iter().enumerate() {
                masked_payload.push(byte ^ masking_key[i % 4]);
            }

            frame.extend_from_slice(&masked_payload);
        } else {
            frame[1] |= value.len as u8;

            if value.payload.len() >= 4 {
                frame.extend_from_slice(&value.payload);
            }
        }
    }

    /// Adds first byte to the frame
    ///
    /// The first byte is constructed of the FIN bit, RSV1, RSV2, RSV3, and opcode.
    ///
    fn add_first_byte(value: &FrameBuilder, frame: &mut Vec<u8>) {
        frame.push(
            0b10000000
                | (value.fin as u8) << 7
                | (value.rsv1 as u8) << 6
                | (value.rsv2 as u8) << 5
                | (value.rsv3 as u8) << 4
                | value.opcode,
        );
    }

    /// Adds second byte to the frame
    ///
    /// The second byte is constructed of the MASK bit and the length of the payload.
    ///
    fn add_second_byte(value: &FrameBuilder, frame: &mut Vec<u8>) {
        frame.push((value.mask as u8) << 7);
    }
}

impl From<FrameBuilder> for Vec<u8> {
    fn from(value: FrameBuilder) -> Self {
        let mut table = Table::new();

        table.set_titles(row!["Building WebSocket frame"]);
        table.add_empty_row();

        table.add_row(row!["First byte data"]);
        table.add_empty_row();

        table.add_row(row!["FIN: ", format!("{}", value.fin)]);
        table.add_row(row!["RSV1: ", format!("{}", value.rsv1)]);
        table.add_row(row!["RSV2: ", format!("{}", value.rsv2)]);
        table.add_row(row!["RSV3: ", format!("{}", value.rsv3)]);
        table.add_row(row!["Opcode: ", format!("{}", value.opcode)]);

        table.add_empty_row();
        table.add_row(row!["Second byte data"]);
        table.add_empty_row();

        table.add_row(row!["Mask: ", format!("{}", value.mask)]);
        table.add_row(row!["Payload Length: ", format!("{}", value.len)]);
        table.add_row(row!["Payload: ", format!("{:?}", value.payload)]);

        table.add_empty_row();
        table.add_row(row!["Masking key: ", format!("{:?}", value.masking_key)]);

        info!("{}", table);

        let mut frame = Vec::new();

        FrameBuilder::add_first_byte(&value, &mut frame);
        FrameBuilder::add_second_byte(&value, &mut frame);
        FrameBuilder::add_extended_length(&value, &mut frame);
        FrameBuilder::add_payload(&value, &mut frame);

        frame
    }
}
