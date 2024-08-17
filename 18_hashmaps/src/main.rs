use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Şevval"), 20);
    scores.insert(String::from("Arda"), 22);
    scores.insert(String::from("Akif"), 20);
    scores.insert(String::from("Anıl"), 11);
    scores.insert(String::from("Nunu"), 29);

    // it doesn't insert because key already exist
    scores.entry(String::from("Anıl")).or_insert(12);

    println!("{:?}", scores.entry(String::from("Akif")));
    for (key, value) in &scores {
        println!("{key}: {value} yaş");
    }

    let i_love_you = "i love love i you love you i you i love you you love you you i";
    let mut map = HashMap::new();

    for word in i_love_you.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{map:?}");
}
