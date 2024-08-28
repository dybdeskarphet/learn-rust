type ColorValue = u8;
type Unsigned64 = u64;
type Inch = u64;

fn main() {
    let red: ColorValue = 252;
    let inches: Inch = 2 as Unsigned64;
   
    println!("{} ColorValue + {} inches = {} unit?",
             red,
             inches,
             red as u64 + inches);
}
