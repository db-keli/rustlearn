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