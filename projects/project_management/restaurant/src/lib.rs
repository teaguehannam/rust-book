use std::fmt::Result;
// providing a keyboard for duplicate
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}


/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     ├── serve_order (super example)
     └── serving
         ├── take_order
         └── take_payment
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn take_payment() {}
    }
}


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super goes to parent module of back_of_house (root)
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Can be used by eat_at_restaurant (because public)
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// ---- importing via use ----
// Absolute path (idiomatic)
use crate::front_of_house::hosting;

// Relative path (idiomatic)
// use self::front_of_house::hosting;

// Absolute path (unidiomatic)
// When called, it's unclear where it's from
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Brought into scope with use
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // -- unidiomatic version --
    // add_to_waitlist();
}


