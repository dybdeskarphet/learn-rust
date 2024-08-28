fn main() {
    let a = 1u8;
    let b = 1u16;
    let c = 1u32;

    println!("size of `a` in bits: {}", std::mem::size_of_val(&a)*8);
    println!("size of `b` in bits: {}", std::mem::size_of_val(&b)*8);
    println!("size of `c` in bits: {}", std::mem::size_of_val(&c)*8);
}
