// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
#[derive(Debug)] // enable debug formatting for struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // ---- Struct in a function ----
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // ---- Debugging ----
    // {:#?} or {:?} in println! will show all values in the field for that instance
    println!("rect1 is {:?}", rect1);

    // ---- Debug within struct ----
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! prints to stderr
    // println! prints to stdout
    dbg!(&rect2);
}


// A struct allows for labels, more clarity than a tuple
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// a tuple is more readable and manageable,
// than passing in individial (width, height) arguments
/* 
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 
}
*/
