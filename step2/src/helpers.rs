
pub mod namehelpers{
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return  full_name;
    }

}

pub mod database {

}

pub mod privatefns {
    pub fn age_plus_5( age: u16) -> u16{
        let new_age = age+ 5;
        return new_age;
    }
}

pub mod statements {
    pub fn test_if(){
        let age_to_drive: u8 = 16u8;

        println!("Enter the person's age: ");
        let input = &mut String::from("");
        std::io::stdin().read_line(input).unwrap();

        let age = input.replace("\n", "").parse::<u8>().unwrap();
        if age  > age_to_drive {
            println!("Issuing driver's lincense");
        }
        else {
            println!("Not allowed to be a driver");
        }
    }
}