mod shapes;
mod formatter;
extern crate rand;

fn main() {
    let shape = shapes::Square {
        x: rand::random::<u8>()%20,
        y: rand::random::<u8>()%20,
    };

    let formatter = formatter::Formatter{ shape, };

    let format = formatter.format();

    println!("{}", format);
}
