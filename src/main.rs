mod shapes;
mod formatter;
use std::net::{TcpListener};
use std::io::Write;
extern crate rand;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:55555").unwrap();
    let mut socket = listener.accept().unwrap();

    let shape = shapes::Square {
        x: rand::random::<u8>()%20,
        y: rand::random::<u8>()%20,
    };
    let formatter = formatter::Formatter{ shape, };
    let result = formatter.format();

    socket.0.write(result.as_bytes()).unwrap();
}
