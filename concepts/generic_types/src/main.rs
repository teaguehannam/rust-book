fn main() {
    // ---- Repetitive example ----
    let num_list1 = vec![34, 50, 25, 100, 64];

    let mut largest1 = num_list1[0];

    for number in num_list1 {
        if number > largest1 {
            largest1 = number;
        }
    }

    println!("Largest number in first list is: {}", largest1);

    // duplicated list
    let num_list1 = vec![102, 34, 9001, 89, 54, 2, 43, 8];

    let mut largest1 = num_list1[0];

    for number in num_list1 {
        if number > largest1 {
            largest1 = number;
        }
    }

    println!("Largest number in first list duplicated is: {}", largest1);

    // ---- Better version ----
    let num_list2 = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&num_list2);
    println!("The largest number is {}", result);

    let num_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&num_list2);
    println!("The largest number is {}", result);

    // ---- Char version ----
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // --------
    // ---- Struct Definitions ----
    let integers = Point { x: 5, y: 10 };
    let floats = Point { x: 1.0, y: 4.2 };
    let integer_and_float = Point { x:5, y: 6.3 };

    // ---- Enum Definitions ----
    let integers = Option_i32::Some(5);
    let floats = Option_f64::Some(5.0);
}

// <T, U> allows x and y be mismatched types
// <T> for both would require x and y to be same types
struct Point<T, U> {
    x: T,
    y: U,
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
