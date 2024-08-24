use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // verbose_error_handling();
    // simple_error_handling();
    // read_username_from_file();
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

/*
fn simple_error_handling() {
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
*/

/*
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

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// `?` can be used for Option results too. In this case, if there is no char, it will return None.
fn last_char_of_first_line(text: &str) -> Option<char> {
    // We used `?` after next becuase next gets the first line
    text.lines().next()?.chars().last()
}

*/

// You're here: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
