use rand::RngExt;
use std::{cmp::Ordering, io};
fn main() {
    let random = rand::rng().random_range(1..=100);
    let mut name = String::new();
    let mut gassing_count = 1;
    loop {
        if name.is_empty() {
            println!("What is your name?");

            io::stdin().read_line(&mut name).unwrap();
            name = name.trim().to_string();
        }
        println!("Hello, {}\nGuess the number:", name);

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let number: u32 = match input.trim().parse() {
            Ok(num) => num,

            Err(_) => {
                println!("Please type a valid number!");

                continue;
            }
        };

        match number.cmp(&random) {
            Ordering::Greater => {
                println!("Too Big");
                gassing_count += 1;
            }

            Ordering::Less => {
                println!("Too Small");
                gassing_count += 1;
            }

            Ordering::Equal => {
                println!("You Win!");
                println!("You guessed the number in {} tries", gassing_count);
                break;
            }
        }
    }

    println!("End of Game");
}
