fn main_v2(x: usize) {
    let repeated = "HELLO WORLD!\n".repeat(x);
    println!("{}", repeated);
}

fn five() -> i8 {
    5
}

fn times_two(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("Hello, world!");
    main_v2(10);
    let x = five();
    println!("Value of x is {x}");
    let y = times_two(10);
    println!("Value of y is {y}");
}
