mod guess;

use guess::Guess;

use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value(),
            Err(_) => continue, // `continue` expression returns never type (`!`) which coerces into any type, including `i32`, and moves the control at the top of the loop; that's why this match expression (whose arms must all have the same type) compiles
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
