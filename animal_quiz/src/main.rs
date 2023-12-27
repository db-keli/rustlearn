pub mod funcs;
use std::io;

fn main() {
    println!("Welcome To Animal Guess Game! ");

    loop{
        let animal = funcs::gen();
        let mut guess = String::new();

        println!("Guess the animal?");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();

        funcs::check(guess, animal);
    }
}
