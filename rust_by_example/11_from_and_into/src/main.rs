// use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}


impl From<i32> for Number {
    /* NOTE: this will also implement `into()` because of this part inside std lib
        impl<T, U> Into<U> for T where U: From<T> {
            fn into(self) -> U {
                U::from(self)
            }
        }
    */

    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let my_number = Number::from(45);
    let int = 20;
    let num: Number = int.into();
    println!("{}", my_string);
    println!("{:?}", num);
    println!("{:?}", my_number);
}
