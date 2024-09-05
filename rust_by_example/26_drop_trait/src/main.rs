#[derive(Debug)]
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "hey" };

    println!("{:?}", _a);

    drop(_a);

    // NOTE: Below code won't work because it's dropped
    // println!("{:?}", _a);
}
