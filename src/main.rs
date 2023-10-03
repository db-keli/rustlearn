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

#[derive(Debug)]
struct Home {
    mom: String,
    dad: String,
    child: String,
}


fn main() {
    //ENUMERATIONS

    let person2 = Cast{
        name: String::from("Aang"),
        nation: Avatar::Air
    };
    println!("{:?}", person2);
    println!("{:?}", person2.nation);

    let person = Home{
        mom: String::from("Daavi"),
        dad: String::from("Sampson"),
        child: String::from("James")
    };

    println!("{}", person.child);
    println!("{:?}", person.child);
}

