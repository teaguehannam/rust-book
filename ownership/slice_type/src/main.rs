fn main() {
    // --------
    let mut s1 = String::from("hello world");

    let word1 = first_word(&s1);

    s1.clear(); // empty the String ("")

    // word still has value based on s,
    println!("word1: {}", word1);

    // --------
    let s2 = String::from("hello world");

    let word2 = first_word_slice(&s2);

    // clear on s2 would cause and error
    // due to adding a second mutable reference
    // s2.clear;

    println!("first word of s2: {}", word2);

}

fn first_word(s: &String) -> usize {
    // convert String to array of bytes
    let bytes = s.as_bytes();

    // iterate over array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // return count once space is found (first word length)
        if item == b' ' {
            return i;
        }
    }

    // no space found, returns length
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // return first word
        if item == b' ' {
            return &s[0..i];
        }
    }

    // no space found, return word
    &s[..]

}

