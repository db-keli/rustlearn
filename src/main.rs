#[derive(Debug)]
enum Payment {
    cash,
    creditcard,
}

enum CarType {
    Hatch,
    Sudan,
    SUV
}

#[derive(Debug)]
struct Pay1 {
    name: String,
    payment: Payment
}

#[derive(Debug)]
enum Avatar {
    Fire,
    Air,
    Water,
    Earth,
}

#[derive(Debug)]
struct Cast {
    name: String,
    nation: Avatar,
}

fn main() {
    //ENUMERATIONS
    let person1 = Pay1{
        name: String::from("Mike"),
        payment: Payment::cash
    };

    println!("{:?}", person1);

    print_size(CarType::SUV);

    let person2 = Cast{
        name: String::from("Aang"),
        nation: Avatar::Air
    };
    println!("{:?}", person2);
}

fn print_size(car:CarType) {
    match car {
        CarType::Hatch => {
            println!("Small size car");
        },
        CarType::Sudan => {
            println!("Medium sized car");
        },
        CarType::SUV => {
            println!("Large sized sports utility car");
        }

    }
}





// fn add(card: Payment, money: Payment){
//
// }

