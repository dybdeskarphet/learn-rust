fn main() {
    let str1 = String::from("hey!");
    let len = calculate_length(&str1);
    println!("Length of {str1}: {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
