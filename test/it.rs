#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::net::{TcpStream};
    extern crate serde;
    extern crate serde_json;

    #[derive(serde::Deserialize, Debug)]
    struct ShapeResult {
        spec: HashMap<String, u8>,
        area: u16,
    }

    #[test]
    fn writes_square_on_connection() {
        let stream = TcpStream::connect("hellorust:55555").unwrap();
        let result: ShapeResult = serde_json::from_reader(stream).unwrap();
        let expected: u16 = (result.spec["x"] * result.spec["y"]).into();
        let actual = result.area;

       assert_eq!(expected, actual);
    }
}
