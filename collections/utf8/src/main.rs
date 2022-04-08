fn main() {
    // ---- Combining Strings ----
    let s1 = String::from("Hello, ");

    let s2 = String::from("world!");

    // s1 is moved here, can no longer be used
    // &s2 points to that spot in memory
    let s3 = s1 + &s2;

    println!("s3: {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    // This takes ownership
    // let s7 = s4 + "-" + &s5 + "-" + &s6;

    // "format!" doesn't take ownership
    let s7 = format!("{}-{}-{}", s4, s5, s6);

    println!("s7: {}", s7);

    // ---- Slicing String ----

    let hello = "Здравствуйте";

    // slice 4 bytes of the string (2 letters)
    let s9 = &hello[0..4];
    println!("slice of hello: {}", s9);

    // ---- Iterating over String ----

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}
