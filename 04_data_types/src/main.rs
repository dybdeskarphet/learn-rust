use std::io;

fn main() {
    let x: f32 = 1.123456789;
    let y = 1.1234567890123456;

    println!("Single precision (f32): {}", x);
    println!("Double precision (f64): {}", y);

    let _a = 'A';
    let _b: char = 'B';
    let duck = '🦆';

    println!("{duck}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x is: {x}");
    println!("y is: {y}");
    println!("z is: {z}");
    println!("First element of tup: {}", tup.0);

    let arr = [3; 5];
    println!("Elements of arr: {:?}", arr);

    let numbers: [i8; 6] = [21, 43, 99, 1, 60, 57];

    println!("Enter an index (max {}):", numbers.len() - 1);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index should be a number");

    let element = numbers[index];

    println!("{element} is at index {index}");
}
