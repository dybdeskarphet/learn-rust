fn main() {
    // coins()
    let five = Some(5);
    let ten = times_two(five);
    let none = times_two(None);
    println!("{:?}", ten);
    println!("{:?}", none);
}

fn times_two(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

fn coins() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Penny);
}

// You're here: https://doc.rust-lang.org/stable/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder