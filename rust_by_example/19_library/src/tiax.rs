pub fn public_function() {
    println!("This is a public function.");
}

fn private_function() {
    println!("This is a private function.");
}

pub fn indirect_access() {
    println!("Accessing a private function...");
    private_function();
}
