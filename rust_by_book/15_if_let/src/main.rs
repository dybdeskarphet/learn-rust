fn main() {
    let config_max = Some(3u8);
    if let Some(x) = config_max {
        println!("The maximum is configred to be {x}");
    } else {
        println!("")
    }
}
