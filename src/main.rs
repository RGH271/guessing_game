// import the io library from "standard"
// this allows for user input
use std::io;
// import Rng from rand to enable random numbers
use rand::Rng;


// declare the main function
fn main() {
    // print some instruction strings
    println!("Guess the number!");

    // set a variable for the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");


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


    println!("You guessed: {}", guess);
}