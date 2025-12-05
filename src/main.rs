use std::{io::Read, net::TcpStream, process::exit, thread::sleep, time::Duration};

use log::{error, info};

mod config;
mod frame;
mod handshake;
mod payload;

use config::Args;

use clap::Parser;

use crate::handshake::Handshake;
use payload::Payload;

fn get_stream() -> TcpStream {
    let args = Args::parse();

    let socket_addr = format!("{}:{}", args.address, args.port);

    info!("Connecting to {}", socket_addr);

    TcpStream::connect(socket_addr).unwrap_or_else(|f| {
        error!("Connection failed! Error: {f:?}");

        exit(0)
    })
}

fn set_write_timeout(stream: &mut TcpStream) {
    stream
        .set_write_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap_or_else(|_| {
            error!("Failed to set write timeout!");

            exit(0)
        });
}

fn disable_nagle(stream: &mut TcpStream) {
    stream.set_nodelay(true).unwrap_or_else(|_| {
        error!("Failed to set nodelay!");

        exit(0)
    });
}

fn perform_handshake(stream: TcpStream) {
    let args = Args::parse();

    info!(
        "Performing WebSocket handshake with headers
Sec-WebSocket-Key: {}
Host: {}:{}
GET {} HTTP/1.1
",
        args.sec_websocket_key, args.address, args.port, args.path
    );

    let mut handshake = Handshake::from(stream.try_clone().unwrap());

    handshake.finish();
}

fn wait_confirm(stream: &mut TcpStream) {
    let mut buf = [0u8; 256];

    stream.read(&mut buf).unwrap_or_else(|_| {
        error!("Failed to read confirmation ! ! ! The server may be down");

        exit(0)
    });

    info!(
        "Confirmation completed: {:?}",
        String::from_utf8_lossy(&buf)
    );
}

fn send_payload(stream: &mut TcpStream) {
    let mut payload = Payload::new(stream.try_clone().unwrap());

    payload.send();

    info!("Payload sent");
}

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let mut stream = get_stream();

    disable_nagle(&mut stream);
    set_write_timeout(&mut stream);
    perform_handshake(stream.try_clone().unwrap());
    wait_confirm(&mut stream);
    send_payload(&mut stream);

    loop {
        wait_confirm(&mut stream);

        info!("Waiting. . .");

        sleep(Duration::from_millis(500));
    }
}
