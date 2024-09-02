trait Greet {
    fn say_hello(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

fn main() {
    let person = Person {
        name: String::from("Arda"),
    };

    person.say_hello();

    let temp1 = Centimeters(20.0);
    let temp2 = Centimeters(20.0);
    let temp_nan = Centimeters(f64::NAN);

    assert!(temp1 == temp2);
    assert!(temp_nan != temp_nan);
    // NOTE: NaN is not equal to itself too.

    // NOTE: You're here https://doc.rust-lang.org/rust-by-example/trait/dyn.html
}
