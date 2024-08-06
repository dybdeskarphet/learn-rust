fn main() {
    verify_age(17);
    verify_age(18);
    // again();
    weird_loop();
    while_conditinal();
    list_elements();
    final_countdown();
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

fn while_conditinal() {
    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("GO!");
}

fn list_elements() {
    let array: [u8; 7] = [20, 53, 34, 61, 94, 38, 10];

    for number in array {
        println!("{number}");
    }
}

fn final_countdown() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!!");
}
