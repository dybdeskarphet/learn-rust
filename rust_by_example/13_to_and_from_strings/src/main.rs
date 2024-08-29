use std::fmt;

struct Circle {
    radius: u32
}

// NOTE: With this implementation, string conversion of a Circle can be whatever we want.
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 10 };
    println!("{}", circle.to_string());

    let int1: i32 = "20".parse().unwrap();
    let int2: i32 = "30".parse::<i32>().unwrap();

    println!("Sum of int1 and int2 (parsed): {}", int1 + int2);
}
