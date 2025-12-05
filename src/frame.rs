use clap::Parser;
use log::{error, info};

use crate::config::Args;

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
    pub(crate) fn new() -> Self {
        let args = Args::parse();

        let key: Result<[u8; 4], _> = args.key.as_slice().try_into();

        Self {
            fin: args.fin,
            rsv1: args.rsv1,
            rsv2: args.rsv2,
            rsv3: args.rsv3,
            opcode: args.opcode,
            mask: args.mask,
            len: args.len,
            extended_length_if_126_or_127: args.extended_length,
            masking_key: if key.is_ok() && args.mask {
                Some(key.unwrap())
            } else {
                None
            },
            payload: if args.payload.len() == 0 && args.payload[0] == 0 {
                Vec::new()
            } else {
                args.payload
            },
        }
    }
}

impl Into<Vec<u8>> for FrameBuilder {
    fn into(self) -> Vec<u8> {
        info!(
            "Building WebSocket frame

== First byte data ==
FIN: {}
RSV1: {}
RSV2: {}
RSV3: {}
Opcode: {}

== Second byte data ==
Mask: {}
Payload Length: {}
Payload: {:?}

== Miscellaneous ==

Masking key: {:?}
",
            self.fin,
            self.rsv1,
            self.rsv2,
            self.rsv3,
            self.opcode,
            self.mask,
            self.len,
            self.payload,
            self.masking_key
        );

        /*
         * The first byte should be
         *
         * 0b10000000 | (self.fin as u8) << 7 | (self.rsv1 as u8) << 6 | (self.rsv2 as u8) << 5 | (self.rsv3 as u8) << 4 | self.opcode
         */
        let mut frame = Vec::new();

        frame.push(
            0b10000000
                | (self.fin as u8) << 7
                | (self.rsv1 as u8) << 6
                | (self.rsv2 as u8) << 5
                | (self.rsv3 as u8) << 4
                | self.opcode,
        );

        frame.push((self.mask as u8) << 7);

        if self.len == 126 {
            frame.push((self.extended_length_if_126_or_127 >> 8) as u8);
            frame.push(self.extended_length_if_126_or_127 as u8);
        } else if self.len == 127 {
            frame.push((self.extended_length_if_126_or_127 >> 56) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 48) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 40) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 32) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 24) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 16) as u8);
            frame.push((self.extended_length_if_126_or_127 >> 8) as u8);
            frame.push(self.extended_length_if_126_or_127 as u8);
        }

        if self.mask && self.payload.len() >= 4 {
            let masking_key = match self.masking_key {
                Some(key) => key,
                None => {
                    error!("Masking key not set in FrameBuilder config!");

                    unreachable!()
                }
            };

            frame.extend_from_slice(&masking_key);

            let mut masked_payload = Vec::with_capacity(self.payload.len());

            for (i, byte) in self.payload.iter().enumerate() {
                masked_payload.push(byte ^ masking_key[i % 4]);
            }

            frame.extend_from_slice(&masked_payload);
        } else {
            frame[1] |= self.len as u8;

            if self.payload.len() >= 4 {
                frame.extend_from_slice(&self.payload);
            }
        }

        frame
    }
}
