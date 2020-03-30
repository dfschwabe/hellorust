#[cfg(test)]
mod test {
    use std::net::{TcpStream};
    use std::io::Read;

    #[test]
    fn writes_shape_on_connection() {
        let mut stream = TcpStream::connect("hellorust:55555").unwrap();
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        assert!(buffer.starts_with("Shape"));
    }
}
