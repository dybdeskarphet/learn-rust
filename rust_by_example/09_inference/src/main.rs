fn main() {
    let elem = 5usize;

    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec.what_type);
    // As you can see in the error, vec is a vector of 'usize's
}
