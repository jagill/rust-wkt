use crate::char_class::CharClass;

pub struct Tokenizer<'a> {
    text: &'a str,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokenizer { text, index: 0 }
    }

    pub fn next_non_whitespace(&mut self) -> Option<<Self as Iterator>::Item> {
        for (char_class, token) in self {
            if char_class != CharClass::Whitespace {
                return Some((char_class, token));
            }
        }
        None
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (CharClass, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let mut end = self.index;
        let mut current_class: Option<CharClass> = None;

        while let Some(ch) = self.text.chars().nth(end) {
            let next_class = CharClass::of(ch).unwrap();
            match current_class {
                None => current_class = Some(next_class),
                Some(class) if class == next_class => end += 1,
                Some(_) => break,
            }
        }
        let class = current_class.take()?;
        let token = &self.text[self.index..end];
        self.index = end;

        Some((class, token))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let mut tokens = Tokenizer::new("POINT(1.234, 1234.)");
        assert_eq!(tokens.next().unwrap(), (CharClass::Character, "POINT"));
        assert_eq!(tokens.next().unwrap(), (CharClass::LeftParen, "("));
        assert_eq!(tokens.next().unwrap(), (CharClass::Digit, "1.234"));
        assert_eq!(tokens.next().unwrap(), (CharClass::Comma, ","));
        assert_eq!(tokens.next().unwrap(), (CharClass::Whitespace, " "));
        assert_eq!(tokens.next().unwrap(), (CharClass::Digit, "1234."));
        assert_eq!(tokens.next().unwrap(), (CharClass::RightParen, ")"));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_non_whitespace() {
        let mut tokens = Tokenizer::new("   X  Y   () ");
        assert_eq!(
            tokens.next_non_whitespace().unwrap(),
            (CharClass::Character, "X")
        );
        assert_eq!(
            tokens.next_non_whitespace().unwrap(),
            (CharClass::Character, "Y")
        );
        assert_eq!(
            tokens.next_non_whitespace().unwrap(),
            (CharClass::LeftParen, "(")
        );
        assert_eq!(
            tokens.next_non_whitespace().unwrap(),
            (CharClass::RightParen, ")")
        );
        assert_eq!(tokens.next_non_whitespace(), None);
    }
}
