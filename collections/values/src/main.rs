fn main() {
    // --------
    let v1: Vec<i32> = Vec::new();

    // i32 infered from from initial type
    let v1 = vec![1, 2, 3];

    // --------
    // update vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // --------
    // read elements of vector
    let v3 = vec![1, 2, 3, 4, 5, 6];
    let third: &i32 = &v3[2];
    println!("Third element of v3: {}", third);

    // --------
    // get element using match
    match v3.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element"),
    }

    // --------
    // iterating and changing values
    let mut v4 = vec![100, 32, 56];
    for i in &mut v4 {
        *i += 50;
    }

    // --------
    // Enum with multiple types

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
