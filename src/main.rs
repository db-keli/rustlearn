#[derive(Debug)]
enum Payment {
    cash,
    creditcard,
}

#[derive(Debug)]
struct Pay1 {
    name: String,
    payment: Payment
}

fn main() {
    //ENUMERATIONS
    let person1 = Pay1{
        name: String::from("Mike"),
        payment: Payment::cash
    };

    println!("{:?}", person1);
}



// fn add(card: Payment, money: Payment){
//
// }

