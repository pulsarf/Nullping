use std::{io::Write, net::TcpStream, process::exit};

use log::{error, info};

use clap::Parser;

use crate::Args;

pub(crate) struct Handshake {
    stream: TcpStream,
}

impl Handshake {
    pub fn from(stream: TcpStream) -> Self {
        Handshake { stream: stream }
    }

    pub fn finish(&mut self) {
        let args = Args::parse();

        let message = format!(
            "GET {} HTTP/1.1\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Key: {}\r\n\
Connection: Upgrade\r\n\
Upgrade: websocket\r\n\
Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits\r\n\
Host: {}:{}\r\n\
\r\n",
            args.path, args.sec_websocket_key, args.address, args.port
        );

        info!("Sending handshake message: {message}");

        self.stream
            .write_all(message.as_bytes())
            .unwrap_or_else(|_| {
                error!("Failed to initiate websocket handshake!");

                exit(0);
            });
    }
}
