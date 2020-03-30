#[cfg(test)]
extern crate mockall;
use std::collections::HashMap;

#[cfg_attr(test, mockall::automock)]
pub trait Shape {
    fn spec(&self) -> HashMap<String, u8>;
    fn area(&self) -> u16;
}

pub struct Square { pub x: u8, pub y: u8, }

impl Shape for Square {
    fn spec<'a>(&self) -> HashMap<String, u8> {
        [
            ("x".to_string(), self.x),
            ("y".to_string(), self.y),
        ].iter().cloned().collect()
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
        let mut expected : HashMap<String, u8> = HashMap::new();
        expected.insert("x".to_string(), x);
        expected.insert("y".to_string(), y);

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