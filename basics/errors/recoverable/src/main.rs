// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    // Match possible Result<T, E>
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Can also use expect when importing the file
    // Harder to debug if using .expect() multiple times
    let f = File::open("hello.txt").expect("Failed to open hello.txt");



}