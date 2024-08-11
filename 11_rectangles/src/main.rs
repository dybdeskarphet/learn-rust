fn main() {
    find_area_normal();
    find_area_tuple();
    find_area_struct();
}

fn find_area_normal() {
    let width = 50;
    let height = 60;

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    println!(
        "(simple) The area of the rectangle is {}px^2",
        area(width, height)
    )
}

fn find_area_tuple() {
    let rect = (12, 15);

    fn area(dims: (u32, u32)) -> u32 {
        dims.0 * dims.1
    }

    println!("(tuple) The area of the rectangle is {}px^2", area(rect))
}

fn find_area_struct() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectnagle: &Rectangle) -> u32 {
        rectnagle.width * rectnagle.height
    }

    let rect = Rectangle {
        width: 29,
        height: 4,
    };

    println!("(struct) The area of the rectangle is {}px^2", area(&rect));

    println!("rect is {:#?}", &rect);
}
