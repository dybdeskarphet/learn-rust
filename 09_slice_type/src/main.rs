fn main() {
    let s = String::from("Merhaba dünya!");
    let word = first_word(&s);
    println!("{}", word);

    let arr = [1, 2, 3, 4, 5, 6];
    let slice: &[i8] = &arr[3..5];
    // let slice2: &[i32] = &arr[0..3];

    println!("{:?}", slice)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
