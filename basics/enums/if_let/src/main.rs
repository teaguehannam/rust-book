fn main() {
    // ---- Match only if exists ----
    let config_max = Some(3u8);

    // Way to not have to match all possibilities 
    if let Some(max) = config_max {
        println!("Maximum config is {}", max);
    }
    /* Above is shorthand for:
    match config_max {
        Some(max) => println!("Maximum config is {}", max),
        _ => (), // runs if no matches
    }
    */

    // ---- Match if else ----
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a match");
    }

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
