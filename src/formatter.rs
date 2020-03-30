use shapes;
extern crate serde_json;

pub struct Formatter<T> {
   pub shape: T,
}

impl<T: shapes::Shape> Formatter<T> {
    pub fn format(&self) -> String {
        serde_json::json!({
            "spec": self.shape.spec(),
            "area": self.shape.area(),
        }).to_string()
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
        let expected = serde_json::json!({
            "spec": expected_spec,
            "area": expected_area,
        }).to_string();
        let mut mock_shape = shapes::MockShape::new();
        mock_shape.expect_spec().return_const(expected_spec);
        mock_shape.expect_area().return_const(expected_area);

        let under_test = Formatter{ shape: mock_shape, };
        let actual = under_test.format();

        assert_eq!(expected, actual);
    }
}