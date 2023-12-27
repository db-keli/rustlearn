use rand;
use rand::Rng;

pub fn check(guess: &str, valid: &str){
    if valid.contains(&guess) {
        println!("Correct Answer");
    }
    else {println!("Wrong!")};
}

pub fn gen() -> &'static str {
    let animal: [&str; 4] = ["Dog", "Penguin", "Mouse", "Panda" ];

    let rand_index =rand::thread_rng().gen_range(0, animal.len());
    let rand_animal = animal[rand_index];

    return rand_animal;
}
