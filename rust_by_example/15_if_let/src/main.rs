// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {
    Bar,
    BarBar,
}

fn main() {
    let a = Foo::BarBar;
    let b = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }
}
