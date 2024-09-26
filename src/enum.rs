enum Shape{
    Rectangle,
    Triangle,
    Circle
}

fn main() {
    let shape = Shape::Rectangle;
    match shape {
        Shape::Rectangle => println!("Rectangle"),
        Shape::Triangle => println!("Triangle"),
        Shape::Circle => println!("Circle"),
    }
}

fn area(shape: Shape, width: f64, height: f64) -> f64 {
    match shape {
        Shape::Rectangle => width * height,
        Shape::Triangle => (width * height) / 2.0,
        Shape::Circle => 3.14 * (width * width),
    }
}

fn perimeter(shape: Shape, side: f64) -> f64 {
    match shape {
        Shape::Rectangle => 2.0 * (side + side),
        Shape::Triangle => 3.0 * side,
        Shape::Circle => 2.0 * 3.14 * side,
    }
}