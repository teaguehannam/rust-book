fn main() {
   // ---- Borrowing ----
   let s1 = String::from("hello");
   
   // Reference s1 without taking ownership (&)
   let len = calculate_length(&s1);

   println!("The length of '{}' is {}.", s1, len);

   // ---- Mutable borrowing ----
   let mut s2 = String::from("hello");

   change(&mut s2);

   println!("{}", s2);

   // ---- Mutable references ----
   let mut s3 = String::from("hello");

   /* Can't have simultaneous pointers to the same data
        let r1 = &mut s3;
        let r2 = &mut s3;
    */

    let r1 = &s3;
    let r2 = &s3;
    println!("Immutable shared pointers {} and {}!", r1, r2);

    let r3 = &mut s3;
    println!("Single mutable pointer {}", r3);

    // ---- Dangling References ----
    /* Compiler doesn't allow reference to location,
        that is owned by someone else
        let r4 = dangle();
    */
    let r4 = no_dangle();

    println!("no dangle: {}", r4);

}

// expect s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}

// borrows reference to string,
// and allows for changing
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}