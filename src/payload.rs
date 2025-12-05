use std::{io::Write, net::TcpStream};

use log::info;

use crate::frame::FrameBuilder;

fn generate_frame() -> Vec<u8> {
    let frame = FrameBuilder::new();

    frame.into()
}

pub(crate) struct Payload {
    stream: TcpStream,
}

impl Payload {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

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
