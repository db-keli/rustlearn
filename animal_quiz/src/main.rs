pub mod funcs;
use std::io;

fn main() {
    println!("Welcome To Animal Guess Game! ");

    let answer = "Dog";

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    funcs::check(guess, answer);
}
