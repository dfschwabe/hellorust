mod shapes;

fn print_area<T: shapes::HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
    let s = shapes::Square {
        x: 4,
        y: 6,
    };

   print_area(s); 
}