// Make use of an external dependency called "rand"
extern crate rand; // no need to "use rand;", this is done here.

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Create a mutable binding to a new String.
        let mut guess = String::new();

        // read_line takes a mutable string (&mut String) as an arg.
        // i.e. ("&mut guess" instead of "&guess")
        io::stdin().read_line(&mut guess)
            // read_line() returns an io::Result. io::Result has an ok() method.
            .ok() // Assume this value is a successful one. If not, ignore error info.
            // ok() returns a type with an expect() method.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Right hand side becomes the number that got parsed.
            Err(_) => continue, // Use "_" to indicate we are ignoring this value.
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
