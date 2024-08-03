fn main() {
    verify_age(17);
    verify_age(18);
    // again();
    weird_loop();
}

fn verify_age(age: i32) {
    // it couldn't be false (boolean) and "true" (string)
    // they should be the same type
    let is_restricted = if age < 18 { false } else { true };

    println!("Can we sell cigarattes to a(n) {age}-year-old: {is_restricted}");
}

fn again() {
    loop {
        println!("again!");
    }
}

fn weird_loop() {
    let mut count = 0;
    'counting: loop {
        println!("Counted {count}");
        let mut remaining = 10;

        loop {
            println!("{remaining} remains");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Counting ended with {count}");
}

//  you're here https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#conditional-loops-with-while
