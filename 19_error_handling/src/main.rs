use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //    verbose_error_handling();
    simple_error_handling();
}

fn simple_error_handling() {
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn verbose_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// You're here: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
