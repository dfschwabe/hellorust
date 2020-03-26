pub trait HasArea {
    fn area(&self) -> i32;
}

pub struct Square {
    pub x: i32,
    pub y: i32,
}

impl HasArea for Square {
    fn area(&self) -> i32 {
        self.x * self.y
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let s = Square{ x:7, y:3};

        assert_eq!(21, s.area());
    }
}