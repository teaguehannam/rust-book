fn main() {
    // --------
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // --------
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // --------
    let s3 = gives_ownership();
    let s4 = String::from("hello"); 
    let s5 = takes_and_gives_back(s4);

    // --------
    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
