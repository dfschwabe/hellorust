use shapes;

pub struct Formatter<T> {
   pub shape: T,
}

impl<T: shapes::Shape> Formatter<T> {
    pub fn format(&self) -> String {
        format!("Shape: {:?}\nArea: {}", self.shape.spec(), self.shape.area())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format() {
        let expected_area : u16 = 456;
        let expected_spec : HashMap<&'static str, u8> = 
            [("a", 12),
            ("b", 23),
            ("c", 34)].iter().cloned().collect();
        let expected = format!("Shape: {:?}\nArea: {}", expected_spec, expected_area);
        let mut mock_shape = shapes::MockShape::new();
        mock_shape.expect_spec().return_const(expected_spec);
        mock_shape.expect_area().return_const(expected_area);

        let under_test = Formatter{ shape: mock_shape, };
        let actual = under_test.format();

        assert_eq!(expected, actual);
    }
}