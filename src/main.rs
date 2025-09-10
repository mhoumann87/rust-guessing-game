// Import the libraries you need
use std::io;
// We use the Rand library (crate) to generate a random number
use rand::prelude::*;

fn main() {
    // If you don't know the value of a string  (user input, data base) use the `String::new("")` type
    // If you have a set value use the `&str` type
    // let name: &str = "Michael";

    /*
    // Get the name from the user
    println!("Hey, what's your name? ");
    // Use `mut` when the name will be changed
    let mut name: String = String::new();
    // Store the input from the user
    io::stdin()
        .read_line(&mut name)
        .expect("Error reading input");

    println!("Hello {}, welcome!", name.trim()); // have to remove new line from the return key
    */

    // Let rand generate a random number for us
    let mut rng = rand::rng();
    let correct = rng.random_range(1..=10);
    // Just to debug, we cheat and print the number
    println!("The number are {correct}");
    // We need a number, but we will store it as a string
    println!("Guess a number between 1 and 10");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    // We need to change the string to a number for it to work
    let guess: u32 = guess.trim().parse().expect("Error with parse");

    // We can use an if expression (much like an if statement in other languages)
    if guess == correct {
        println!("You guessed the correct number {}", guess);
    } else {
        println!("You guessed wrong. The correct number was {}", correct);
    }
}
