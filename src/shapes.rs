extern crate mockall;
use std::collections::HashMap;

#[mockall::automock]
pub trait Shape {
    fn spec(&self) -> HashMap<&'static str, u8>;
    fn area(&self) -> u16;
}

pub struct Square { pub x: u8, pub y: u8, }

impl Shape for Square {
    fn spec(&self) -> HashMap<&'static str, u8> {
        let mut expected : HashMap<&'static str, u8> = HashMap::new();
        expected.insert("x", self.x);
        expected.insert("y", self.y); 
        expected
    }

    fn area(&self) -> u16 {
        (self.x * self.y).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec() {
        let x = 7;
        let y = 3;
        let mut expected : HashMap<&'static str, u8> = HashMap::new();
        expected.insert("x", x);
        expected.insert("y", y);

        let under_test = Square{ x:x, y:y };
        let actual = under_test.spec();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_area() {
        let x : u8 = 7;
        let y : u8 = 3;
        let expected : u16 = (x*y).into();

        let under_test = Square{ x:x, y:y };
        let actual = under_test.area();

        assert_eq!(expected, actual);
    }
}