pub fn test_option_type() -> Option<u8>
{
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn test_option_string() -> Option<String>
{
    let mut opt2: Option<String> = None;
    opt2 = Some(String::from("Trevor Sullivan"));

    return opt2;
}