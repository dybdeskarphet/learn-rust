fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    let multiply = |a: i32, b: i32| a * b;
    let divide = |a: i32, b: i32| a / b;
    let add = |a: i32, b: i32| a + b;
    let substract = |a: i32, b: i32| a - b;

    println!("{}", multiply(20, 20));
    println!("{}", add(20, 20));
    println!("{}", divide(20, 20));
    println!("{}", substract(20, 20));

    let int = 420;

    // NOTE: When this one is used, it directly gives us an error if we try to
    // enter different type other than i32
    // let closure_annotated = | i: i32 | -> i32 { i + int };

    // NOTE: What type you use first is going to be the type of the int.
    // Also, if we used the closure_annotated, closure_inferred wouldn't work,
    // because int would already be a i32.
    let closure_inferred = |i| i + int;

    // NOTE: As you can see in the error message, int is a i64.
    println!(
        "closure_inferred: {}",
        closure_inferred(30i64)
    );

    for i in 0..5 {
        if i % 2 == 0 {
            foo()
            // The program will terminate here, so the rest of the loop won't execute.
        }
        println!("Odd number: {}", i);
    }
}
