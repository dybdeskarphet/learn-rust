struct Container<T> {
    value: Option<T>,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self {
            value: Some(value),
        }
    }

    fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
    
    fn update(&mut self, new_value: T) {
        self.value = Some(new_value)
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }

}

fn main() {
    let mut int_container = Container::new(213);
    println!("Value: {:?}", int_container.get());
    
    int_container.update(192);
    println!("After update: {:?}", int_container.get());

    println!("Is empty?: {:?}", int_container.is_empty());
}
