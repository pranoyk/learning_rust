#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = dbg!(Rectangle {
        width: 30,
        height: 50
    });

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(dimensions: &Rectangle) -> u32 {
    return dimensions.width * dimensions.height
}
