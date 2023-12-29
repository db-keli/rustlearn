pub mod helpers;
pub mod closures;
pub mod optiontest;



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
    // let name;
    // let new_age;
    //
    // name = helpers::namehelpers::get_full_name("Mike", "Alonso");
    // println!("Hello from {}", name);
    //
    // new_age = helpers::privatefns::age_plus_5(6);
    // println!("After five years he's, {0}", new_age);
    //
    // helpers::statements::test_if();
    // helpers::statements::test_while();
    //helpers::statements::test_loop();
    // helpers::statements::test_for();
    //
    // closures::test_closures();

    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

   let result =  optiontest::test_option_type();

    println!("{0}", result.unwrap());

    let strresult = optiontest::test_option_string();

    println!("{0}", strresult.unwrap());

    let charresult = optiontest::test_option_chartype();

    println!("{}", charresult.unwrap().to_string());
}




