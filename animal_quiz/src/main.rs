pub mod funcs;
use std::io;

fn main() {
    let answer = "Dog";
    let mut guess = String::new();

    println!("Welcome To Animal Guess Game! ");
    println!("Guess the animal?");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    funcs::check(guess, answer);
}
