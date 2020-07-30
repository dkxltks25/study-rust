#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is{:?}", rect1);
}

fn area(reactangle: &Rectangle) -> u32 {
    reactangle.width * reactangle.length
}