// fn declares new function
// main is entry point of many programs
fn main() {
    println!("Hello, world!");

    // calling outside function
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // expression, not a statement
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let e = plus_one(10);

    println!("The value of e: {}", e);

}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("Measurement is {}{}.", x, unit_label);
}

/*
    function with return value
    declare type after ->
    use "return" to return earlier
    most functions return last expression implicitly
*/
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
