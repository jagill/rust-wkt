use crate::char_class::CharClass;
use crate::parse_error::ParseResult;

pub struct Tokenizer<'s> {
    text: &'s str,
    index: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'s> {
    pub index: usize,
    pub char_class: CharClass,
    pub value: &'s str,
}

impl<'s> Tokenizer<'s> {
    pub fn new(text: &'s str) -> Self {
        Tokenizer { text, index: 0 }
    }

    pub fn next_non_whitespace(&mut self) -> Option<<Self as Iterator>::Item> {
        for result in self {
            if let Ok(Token { char_class, .. }) = &result {
                if char_class == &CharClass::Whitespace {
                    continue;
                }
            }
            return Some(result);
        }
        None
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<'s> Iterator for Tokenizer<'s> {
    type Item = ParseResult<Token<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end = self.index;
        let mut current_class: Option<CharClass> = None;

        while let Some(ch) = self.text.chars().nth(end) {
            let next_class_result = CharClass::of(ch);
            match next_class_result {
                Err(e) => return Some(Err(e)),
                Ok(next_class) => match current_class {
                    None => current_class = Some(next_class),
                    Some(class) if class == next_class => end += 1,
                    Some(_) => break,
                },
            }
        }
        let token_index = self.index;
        let char_class = current_class.take()?;
        let value = &self.text[self.index..end];
        self.index = end;

        Some(Ok(Token {
            index: token_index,
            char_class,
            value,
        }))
    }
}

mod tests {
    use super::*;

    fn assert_token_value(
        result: Option<ParseResult<Token>>,
        expected_char_class: CharClass,
        expected_value: &str,
    ) {
        let Token {
            char_class, value, ..
        } = result.unwrap().unwrap();
        assert_eq!(char_class, expected_char_class);
        assert_eq!(value, expected_value);
    }

    #[test]
    fn test_tokens() {
        let mut tokens = Tokenizer::new("POINT(1.234, 1234.)");
        assert_token_value(tokens.next(), CharClass::Character, "POINT");
        assert_token_value(tokens.next(), CharClass::LeftParen, "(");
        assert_token_value(tokens.next(), CharClass::Digit, "1.234");
        assert_token_value(tokens.next(), CharClass::Comma, ",");
        assert_token_value(tokens.next(), CharClass::Whitespace, " ");
        assert_token_value(tokens.next(), CharClass::Digit, "1234.");
        assert_token_value(tokens.next(), CharClass::RightParen, ")");
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_non_whitespace() {
        let mut tokens = Tokenizer::new("   X  Y   () ");
        assert_token_value(tokens.next_non_whitespace(), CharClass::Character, "X");
        assert_token_value(tokens.next_non_whitespace(), CharClass::Character, "Y");
        assert_token_value(tokens.next_non_whitespace(), CharClass::LeftParen, "(");
        assert_token_value(tokens.next_non_whitespace(), CharClass::RightParen, ")");
        assert_eq!(tokens.next_non_whitespace(), None);
    }
}
