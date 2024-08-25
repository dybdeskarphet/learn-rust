fn main() {
    println!("{} weeks", 7);
    println!("{0} is greater than {1}, {1} is smaller than {0}", 9, 7);
    println!("{word}: {definition}", word="apple", definition="the round fruit of a tree of the rose family, which typically has thin green or red skin and crisp flesh");
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("{number:>5}", number=1); 
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print...", Structure(3));
}
