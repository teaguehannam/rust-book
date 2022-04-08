use std::collections::HashMap;

fn main() {

    // ---- Creating ----
    let mut scores1 = HashMap::new();

    // Adding
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // Create vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Insert vector into HashMap
    let scores1: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Create variables
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // Insert variabales into HashMap
    let mut map1 = HashMap::new();
    map1.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // ---- Accessing values ----
    let team_name = String::from("Blue");
    let team_score = scores1.get(&team_name);

    println!("{:?}", team_score);

    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Yellow"), 50);

    // iterate over map
    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    // ---- Update values ----
    let mut scores3 = HashMap::new();

    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 50);

    println!("{:?}", scores3);

    // ---- Insert if no value ---
    let mut scores4 = HashMap::new();

    scores4.insert(String::from("Blue"), 10);

    // No yellow value, will insert
    scores4.entry(String::from("Yellow")).or_insert(50);
    // Blue already has value, won't change
    scores4.entry(String::from("Blue")).or_insert(50);

    println!("scores4 {:?}", scores4);

    // ---- Insert based on Old Value ----
    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);

}
