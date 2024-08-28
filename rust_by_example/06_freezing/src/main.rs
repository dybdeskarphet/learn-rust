fn main() {
    let mut _some_mutable = 10u8;

    {
        let _some_mutable = _some_mutable;
        // _some_mutable = 20u8;
        // Cannot modify the variable because it is frozen in this scope.
    }

    _some_mutable = 30u8;

    println!("{}", _some_mutable);
}
