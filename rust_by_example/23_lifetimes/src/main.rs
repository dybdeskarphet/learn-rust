fn longest_v1(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_owned()
    } else {
        y.to_owned()
    }
}

fn longest_v2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "short".to_owned();
    let s2 = "longer".to_owned();

    let result_v1 = longest_v1(&s1, &s2);

    println!("Memory address of result_v1 (reference): {:p}", &result_v1);
    println!("Memory address of w1 (reference): {:p}\n", &s1);

    let w1 = "short".to_owned();
    let w2 = "longer".to_owned();

    let result_v2 = longest_v2(&w1, &w2);

    println!("Memory address of result_v2 (reference): {:p}", &result_v2);
    println!("Memory address of w2 (reference): {:p}", &w2);

    println!(
        "Memory address of data referenced by result_v2: {:p}",
        result_v2
    );
    println!("Memory address of data referenced by w2: {:p}", w2.as_str());

    // NOTE: When you run this code, you can see that result_v2 points to w2, but result_v1 doesn't point to s2.
    // This is the use of lifetimes, less memory allocation, but for what? Did we really have to do this?
}
