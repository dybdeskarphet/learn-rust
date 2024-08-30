fn main() {
    let number = 5;

    println!("Tell me about {}", number);
    
    match number {
        1 => println!("HAHA ONE!"),
        2 | 3 | 5 | 7 | 11 => println!("The name is prime-"),
        13..=19 => println!("You're still young"),
        _ => println!("Nothing special about {}", number),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1, 
    };

    println!("{} -> {}", boolean, binary);

    fizzbuzz(30);
}

fn fizzbuzz(n: u32) {
    for n in 1..=n {
        match (n % 3, n % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => println!("{}", n),

        }
    }
}
