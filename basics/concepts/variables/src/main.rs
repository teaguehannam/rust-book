fn main() {
    // ------- Mutability -------
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // ------- Constants -------
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours is: {} seconds", THREE_HOURS_IN_SECONDS);

    // ------- Shadowing -------
    let x = 5;

    // Create shadow version
    let x = x + 1;

    {
        // Create shadow version of the shadowed version
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // outside of the double shadowed versions scope
    println!("The value of x is: {}", x);
}
