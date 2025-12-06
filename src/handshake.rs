use crate::Args;
use clap::Parser;
use log::{error, info};
use std::{fmt::Display, io::Write, net::TcpStream, process::exit};

/// WebSocket handshake descriptor
pub(crate) struct Handshake {
    stream: TcpStream,
}

impl Handshake {
    /// Creates a new handshake instance from a TCP stream.
    ///
    pub fn from(stream: TcpStream) -> Self {
        Handshake { stream }
    }

    /// Creates a new handshake request message.
    ///
    fn create_handshake_request<T>(
        &self,
        path: T,
        sec_websocket_key: T,
        address: T,
        port: u16,
    ) -> String
    where
        T: Into<String> + Display,
    {
        format!(
            "GET {} HTTP/1.1\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Key: {}\r\n\
Connection: Upgrade\r\n\
Upgrade: websocket\r\n\
Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits\r\n\
Host: {}:{}\r\n\
\r\n",
            path, sec_websocket_key, address, port
        )
    }

    /// Writes the handshake request message to the stream.
    ///
    fn write_to_stream(&mut self, message: String) {
        self.stream
            .write_all(message.as_bytes())
            .unwrap_or_else(|_| {
                error!("Failed to initiate websocket handshake!");

                exit(0);
            });
    }

    /// Performs the WebSocket handshake.
    ///
    pub fn finish(&mut self) {
        let args = Args::parse();

        let message = self.create_handshake_request(
            args.server.path,
            args.server.sec_websocket_key,
            args.server.address,
            args.server.port,
        );

        info!("Sending handshake message: {message}");

        self.write_to_stream(message);
    }
}
