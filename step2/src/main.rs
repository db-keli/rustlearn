pub mod helpers;
fn main() {
    // println!("Guess the number");
    // println!("Please input your guess.");
    //
    // let secret = rand::thread_rng().gen_range(1, 101);
    // let mut guess = String::new();
    //
    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");
    //
    // match guess.cmp(&secret){
    //     Ordering::Greater => println!("Too big"),
    //     Ordering::Equal => println!("You Win"),
    //     Ordering::Less => println!("Too small"),
    // }
    // println!("You guessed: {} and real number is {}", guess, secret)
    let name;
    let new_age;

    name = helpers::namehelpers::get_full_name("Mike", "Alonso");
    println!("Hello from {}", name);

    new_age = helpers::privatefns::age_plus_5(6);
    println!("After five years he's, {0}", new_age);
}





