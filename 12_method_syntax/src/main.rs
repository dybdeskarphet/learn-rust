fn main() {
    defining_methods();
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn defining_methods() {
    let rect1 = Rectangle {
        width: 14,
        height: 2,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 2,
    };

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels.",
        // becuase rust has automatic referencing, it is same as (&rect).area()
        rect1.area()
    );

    println!("Height of the rectangel is {} pixels.", rect1.height());
    println!("Width of the rectangel is {} pixels.", rect1.width());
}

// You're here: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#wheres-the---operator
