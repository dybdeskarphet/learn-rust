use std::io;

fn main() {
    println!("Sayıyı tahmin et!");
    println!("Lütfen tahmininizi giriniz:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
