use std::fs::File;
use std::io::Write;
use std::process::Command;

struct Redirection {
    first: String,
    second: String,
}

// struct Piping {
//     first: String,
//     second: String,
// }

impl Redirection {
    fn redirect(command: &str, output_path: &str) {
        let command_output;

        if (command.contains(char::is_whitespace)) {
            let (program, args) = command.split_once(char::is_whitespace).unwrap();
            let args: Vec<&str> = args.split(char::is_whitespace).collect();
            command_output = Command::new(program).args(&args).output().unwrap().stdout;
        } else {
            command_output = Command::new(command).output().unwrap().stdout;
        }

        let mut file = File::create(output_path).expect("Unable to create file");
        let _ = file.write_all(&command_output);
        println!("File is crated at: {}", output_path);
    }
}

fn main() {
    Redirection::redirect("ls -l", "hello.txt")
}
