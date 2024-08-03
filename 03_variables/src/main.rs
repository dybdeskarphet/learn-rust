fn main() {
    // MUTABLE
    let mut x = 5;
    println!("x is: {x}");
    // without mut, we cannot asign
    x = 6;
    println!("x is: {x}");

    // CONSTANTS
    const ONE_DAY_IN_SECONDS: u32 = 60 * 60 * 24;
    println!("One day is {ONE_DAY_IN_SECONDS} seconds");

    // SHADOWING
    let y = 22;
    let y = y + 1;

    {
        let y = y * 2;
        println!("Inner scope y value: {y}");
    }

    println!("y value: {y}");

    let spaces = "            ";
    let spaces = spaces.len();
    // spaces = spaces.len();
    // won't work because spaces is not mutable
    println!("Amount of spaces: {spaces}")
}
