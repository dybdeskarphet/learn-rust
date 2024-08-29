fn main() {
    for n in 1..=100 {
        let mut fizzbuzz = String::from("");

        if n % 3 == 0 {
            fizzbuzz.push_str("fizz");
        }

        if n % 5 == 0 {
            fizzbuzz.push_str("buzz");
        }

        if fizzbuzz.is_empty() {
            println!("{}", n);
        } else {
            println!("{}", fizzbuzz);
        }
    }
}
