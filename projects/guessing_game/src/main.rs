use rand::Rng; // added through Cargo.toml
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // .gen_range(1..101) is the same as .gen_range(1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // Variables are immutable by default, mut allows mutation
        let mut guess = String::new();

        /*  
            & indicates reference (not a copy of the data)
            references immutable by default, so we use mut
            result types are enumerations (enums) known as variants
                Ok means operation successful, and inside is the generated value
                Err means operation failed, and contains information on why
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
            Reuse (shadowing) variable guess instead of creating another
            trim() eliminates whitespace
            parse method on strings turns into number type (u32 here)
            parse() returns result (Ok, Err) 
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
