#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CharClass {
    Whitespace,
    Character,
    LeftParen,
    RightParen,
    Digit,
    Comma,
}

impl CharClass {
    pub fn of(c: char) -> Result<CharClass, String> {
        match c {
            '(' => Ok(CharClass::LeftParen),
            ')' => Ok(CharClass::RightParen),
            ',' => Ok(CharClass::Comma),
            'A'..='z' => Ok(CharClass::Character),
            '0'..='9' | '.' => Ok(CharClass::Digit),
            x if x.is_whitespace() => Ok(CharClass::Whitespace),
            _ => Err(format!("Unknown character '{}'", c)),
        }
    }
}

mod test {
    use super::CharClass;

    #[test]
    fn test_chars() {
        assert_eq!(CharClass::of('a').unwrap(), CharClass::Character);
        assert_eq!(CharClass::of('A').unwrap(), CharClass::Character);
        assert_eq!(CharClass::of('4').unwrap(), CharClass::Digit);
        assert_eq!(CharClass::of('\n').unwrap(), CharClass::Whitespace);
    }
}
