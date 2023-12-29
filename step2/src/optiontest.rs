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

pub fn test_option_chartype() -> Option<CharacterType>
{
    let mut chartype: Option<CharacterType> = None;

    chartype = Some(CharacterType::Mage);

    return chartype;
}

pub enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer".to_string(),
            CharacterType::Mage => "Mage".to_string(),
            CharacterType::Warrior => "Warrior".to_string(),
        }.to_string()
    }
}