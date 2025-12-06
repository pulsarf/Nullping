use crate::frame::FrameBuilder;
use log::info;
use std::{io::Write, net::TcpStream};

/// Generates a frame for the payload
///
/// We need to get a buffer for the payload, and so
/// `Vec<u8>` implements From<FrameBuilder>
///
fn generate_frame() -> Vec<u8> {
    let frame = FrameBuilder::new();

    frame.into()
}

/// Payload descriptor
///
pub(crate) struct Payload {
    stream: TcpStream,
}

impl Payload {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Sends the payload to the server
    ///
    /// We have to flush the socket, since we don't want to get our payload buffered
    /// (and lingered after we close the socket)
    ///
    pub(crate) fn send(&mut self) {
        let frame = generate_frame();

        self.stream.write_all(&frame).unwrap();

        self.stream.flush().unwrap();

        info!(
            "Payload sent successfully

{:x?}",
            frame
        );
    }
}
