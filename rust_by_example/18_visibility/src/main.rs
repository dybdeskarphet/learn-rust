use i::love::you as iloveyou;

mod test {
    fn private() {
        println!("This is a private function.")
    }

    pub fn public() {
        println!("This is a public function.")
    }

    pub fn indirect_access() {
        println!("I'm accessing the private function...");
        private();
    }
}

mod boxes {
    #[derive(Debug)]
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[derive(Debug)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn you() {
    println!("This is just a good old regular function.");
}

mod i {
    pub mod love {
        pub fn you() {
            println!("I love you!")
        }
    }
}

fn main() {
    test::public();
    test::indirect_access();

    let open_box = boxes::OpenBox {
        contents: "hey, i'm an open box",
    };
    println!("{:?}", open_box);

    let closed_box = boxes::ClosedBox::new("hush, this is a secret place");
    println!("{:?}", closed_box);
    // NOTE: Above code is not possible because `contents` is private
    // println!("{:?}", closed_box.contents);

    iloveyou();

    {
        use crate::i::love::you;
        // NOTE: Scope is important!
        you();
    }

    you();
}
