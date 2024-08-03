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

    let _numbers: [i8; 6] = [1, 2, 3, 4, 5, 6];
}
