fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    // let char = decimal as char;

    println!("{}", integer);
    println!("{}", decimal);
    // println!("{}", char);

    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 1583 - 256 - 256 - 256 - 256 - 256 - 256 = 47
    println!("1583 as a u8 is : {}", 1583i16 as u8);

    unsafe {
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
