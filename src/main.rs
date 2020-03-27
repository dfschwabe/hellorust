mod shapes;
mod formatter;
use std::net::{TcpListener};
use std::io::Write;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate rand;


fn main() {
    simple_logger::init().unwrap();
    info!("starting server...");
    let listener = TcpListener::bind("0.0.0.0:55555").unwrap();
    info!("server started, waiting for connection...");

    let mut socket = listener.accept().unwrap();
    info!("connection accepted!");

    let shape = shapes::Square {
        x: rand::random::<u8>()%20,
        y: rand::random::<u8>()%20,
    };
    let formatter = formatter::Formatter{ shape, };
    let result = formatter.format();

    socket.0.write(result.as_bytes()).unwrap();

    info!("fin.");
}
