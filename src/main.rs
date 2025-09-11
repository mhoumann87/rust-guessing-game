// Import the libraries you need
use std::{cmp::Ordering, io};
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
    // println!("The number are {correct}");
    // We need a number, but we will store it as a string
    println!("Guess a number between 1 and 10");

    // We can use an if / else expression (much like an if / else statement in other languages)
    /* if guess < correct {
        println!("You've guessed too low");
    } else if guess > correct {
        println!("You've guessed to high");
    } else {
        println!("You guessed the correct number");
    }*/

    // A "more correct way" to do this is
    /* let message = if guess > correct {
        String::from("You've guessed to high")
    } else if guess < correct {
        String::from("You've guessed to low")
    } else {
        String::from("You guessed correct")
    };
    // Notice that there are no semi colons after the strings, it is after the total expression

    // And then we have to print the correct string
    println!("{}", message); */

    // We want to make the game loop until we get the correct answer
    // We use the `loop` in Rust, and that is an infinite loop, that you have to break out of

    loop {
        // We need to make the variable for each run, else the user input will add to the guess and not replace it, and that gives an error
        let mut guess = String::new();
        //And we need to ask for a new guess each time the game runs
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        // We need to change the string to a number for it to work

        // This is an easy way to get the code running, but the expect gives way to an "panic" and we want to "kill" the chances of introducing errors in our code
        /* let guess: u32 = guess.trim().parse().expect("Error with parse"); */

        // The parse() gives us an enum and we can handle errors using a match

        // We can get rid of this line and put it in the match statement
        /* let mut guess: u32 = guess.trim().parse(); */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };

        // We can also use a match statement in Rust to run the game
        // We need to import `cpm::Ordering` from the std library
        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed to high"),
            Ordering::Less => println!("You guessed to low"),
            Ordering::Equal => {
                println!("You are correct");
                // And here we want to break the loop
                break;
            }
        };
    }
}
