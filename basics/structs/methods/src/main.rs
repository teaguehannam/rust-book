#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block, associated with Rectangle type
// Can have multiple impl blocks on the same type,
// but doesn't make sense to do (yet? ever?)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // ---- Defining methods ----
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // area() is a method of Rectangle type
    );

    // ---- Getter method ----
    if rect1.width() {
        println!("rect1 has a width of {}", rect1.width);
    }

    // ---- Methods with more parameters ----
    println!("rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 hold rect3? {}", rect1.can_hold(&rect3));

}