enum Calculator {
    Add,
    Substract,
    Mulitply,
    Divide
}

impl Calculator {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
            Self::Divide => x / y,
            Self::Mulitply => x * y,
        }
    }
}

fn main() {
    use crate::Calculator::{Add,Substract,Divide,Mulitply};
    let ops = (Add.run(10,40), Substract.run(40,10), Divide.run(63,3), Mulitply.run(33,3));
    println!("{:?}", ops);
}
