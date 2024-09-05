// import the io library from "standard"
// this allows for user input
use std::io;
// import "ordering", an enum type that allows for comparison of two values
use std::cmp::Ordering;
// import Rng from rand to enable random numbers
use rand::Rng;


// declare the main function
fn main() {
    // print some instruction strings
    println!("Guess the number!");

    // set a variable for the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop the game logic forever
    loop {
        println!("Please input your guess.");

        // set a MUTABLE (changable) variable for the guess
        // the "String::new()" part of it declares it as an empty string
        let mut guess = String::new();

        // call the "stdin" function from the "io" module
        io::stdin()
            // print the read line to the guess variable
            .read_line(&mut guess)
            // crash the program and print the message if there is an invalid input
            // rust will give an error on compilation if there is no ".expect", as it adds a layer of saftey
            .expect("Failed to read line");

        // convert guess (a string) into an integer that can be compared
        let guess: u32 = match guess
            // remove any whitespace from "guess"
            .trim()
            // parse converts a string to another type
            .parse() {
                // use match now instead of expect so the program doesn't crash
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);

        // use "Ordering" to compare the two values
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

// test!!