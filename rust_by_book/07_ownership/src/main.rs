fn main() {
    ownership1();
    let str = String::from("hello");
    takes_ownership(str);
    // this won't work because str's value is moved to the function
    // println!("{str}");

    let x = 6;
    makes_copy(x);
    println!("{x}");
}

fn takes_ownership(str: String) {
    println!("{str}");
}

fn makes_copy(x: i32) {
    println!("{x}")
}

fn ownership1() {
    let x = 5;
    let y = x;

    println!("Adress of x: {:p}", &x);
    println!("Adress of y: {:p}", &y);

    // s1 moved to s2
    let s1 = String::from("Hello");
    let s2 = s1;

    // because we moved s1 to s2, s1 is no longer valid,
    // therefore we cannot print it or do anything with it.
    println!("Adress of s2: {:p}", &s2);

    // t1 copied to t2
    let t1 = String::from("Hello");
    let t2 = t1.clone();

    // because we copied t1 to t2, we can do things with both
    println!("Adress of t1: {:p}", &t1);
    println!("Adress of t2: {:p}", &t2);
}
