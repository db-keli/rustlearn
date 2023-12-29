
pub mod namehelpers{
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return  full_name;
    }

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

    pub fn test_while(){
        let age_to_drive = 16u8;

        let mut current_age = 0u8;

        while current_age < age_to_drive {
            println!("Waiting ...");
            current_age += 1;
        }
    }

    pub fn test_loop(){
        let mut x = 1;
        loop {
            println!("Hello from rust! ");

            if x > 5{
                break;
            }
            x += 1;
        }
    }

    pub fn test_for(){
        let ages = [ 34, 53, 67 ,41, 14];
        let age_to_drive = 16i32;

        for value in ages {
            println!("The current age is {0}", value);
            if value > age_to_drive {
                println!("You're old enough to drive");
            }
            else {
                println!("You need to wait a little more.");
            }
        }
    }
}