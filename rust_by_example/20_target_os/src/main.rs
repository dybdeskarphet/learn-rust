#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You're on Linux.")
}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You're on Linux.")
}

fn main() {
    are_you_on_linux();

    println!("You're not sure? Okay, testing again...");

    if cfg!(target_os = "linux") {
        println!("Yes, you're definitely on Linux!");
    } else {
        println!("Sorry, mb, you're not on Linux.");
    }
}
