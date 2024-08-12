fn main() {
    defining_methods();
}

fn defining_methods() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn width(&self) -> u32 {
            self.width
        }
    }

    impl Rectangle {
        fn height(&self) -> u32 {
            self.height
        }
    }

    let rect = Rectangle {
        width: 14,
        height: 2,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Height of the rectangel is {} pixels.", rect.height());
    println!("Width of the rectangel is {} pixels.", rect.width());
}

// You're here: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#wheres-the---operator
