use crate::char_class::CharClass;
use crate::parse_error::{ParseError, ParseResult};
use crate::tokenizer::{Token, Tokenizer};

struct CoordinateReader<'t> {
    coordinates: Vec<f64>,
    tokenizer: &'t mut Tokenizer<'t>,
    state: CoordinateState,
}

impl<'t> CoordinateReader<'t> {
    pub fn new(tokenizer: &'t mut Tokenizer<'t>) -> Self {
        CoordinateReader {
            coordinates: Vec::new(),
            tokenizer,
            state: CoordinateState::Start,
        }
    }

    pub fn coordinates(&self) -> &[f64] {
        &self.coordinates
    }

    pub fn state(&self) -> CoordinateState {
        self.state
    }

    fn read_token(&mut self) -> ParseResult<Token<'t>> {
        match self.tokenizer.next_non_whitespace() {
            None => Err(ParseError::empty_coordinate_sequence(
                self.tokenizer.index(),
            )),
            Some(Err(e)) => Err(e),
            Some(Ok(token)) => Ok(token),
        }
    }

    pub fn read(&mut self) -> ParseResult<()> {
        let token = self.read_token()?;
        match token.char_class {
            CharClass::LeftParen => {
                self.require_state(CoordinateState::Start, &token)?;
                self.state = CoordinateState::ReadingFirstCooordinate;
                Ok(())
            }
            CharClass::RightParen => {
                if self.state == CoordinateState::ReadingFirstCooordinate
                    || self.state == CoordinateState::FinishedCoordinate
                {
                    self.state = CoordinateState::Finished;
                    Ok(())
                } else {
                    Err(ParseError::invalid_token(&token))
                }
            }
            CharClass::Character => {
                self.require_state(CoordinateState::Start, &token)?;
                if token.value.to_ascii_lowercase() != "empty" {
                    Err(ParseError::invalid_token(&token))
                } else {
                    self.state = CoordinateState::Finished;
                    Ok(())
                }
            }
            CharClass::Digit => {
                if self.state == CoordinateState::ReadingFirstCooordinate
                    || self.state == CoordinateState::ReadingNextCoordinate
                {
                    match token.value.parse::<f64>() {
                        Err(_) => Err(ParseError::invalid_token(&token)),
                        Ok(f) => {
                            self.coordinates.push(f);
                            if self.state == CoordinateState::ReadingFirstCooordinate {
                                self.state = CoordinateState::ReadingNextCoordinate;
                            } else if self.state == CoordinateState::ReadingNextCoordinate {
                                self.state = CoordinateState::FinishedCoordinate;
                            }
                            Ok(())
                        }
                    }
                } else {
                    Err(ParseError::invalid_token(&token))
                }
            }
            CharClass::Comma => {
                self.require_state(CoordinateState::FinishedCoordinate, &token)?;
                self.state = CoordinateState::ReadingFirstCooordinate;
                Ok(())
            }
            _ => Err(ParseError::invalid_token(&token)),
        }
    }

    fn require_state(&self, state: CoordinateState, token: &Token) -> ParseResult<()> {
        if self.state != state {
            Err(ParseError::invalid_token(token))
        } else {
            Ok(())
        }
    }

    fn require_state_not(&self, state: CoordinateState, token: &Token) -> ParseResult<()> {
        if self.state == state {
            Err(ParseError::invalid_token(token))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CoordinateState {
    Start,
    ReadingFirstCooordinate,
    ReadingNextCoordinate,
    FinishedCoordinate,
    Finished,
}

mod tests {
    use super::*;

    fn read_all(reader: &mut CoordinateReader) -> ParseResult<()> {
        // println!("{:?} : {:?}", reader.state(), reader.coordinates());
        while reader.state() != CoordinateState::Finished {
            reader.read()?;
            // println!("{:?} : {:?}", reader.state(), reader.coordinates());
        }
        Ok(())
    }

    #[test]
    fn test_empty() {
        let s = "empty";
        let mut tokenizer = Tokenizer::new(s);
        let mut reader = CoordinateReader::new(&mut tokenizer);
        read_all(&mut reader).unwrap();
        assert_eq!(reader.coordinates(), &[]);
    }

    #[test]
    fn test_error() {
        let s = "emptier";
        let mut tokenizer = Tokenizer::new(s);
        let mut reader = CoordinateReader::new(&mut tokenizer);
        assert!(reader.read().is_err());
    }

    #[test]
    fn test_empty_seq() {
        let s = "( )";
        let mut tokenizer = Tokenizer::new(s);
        let mut reader = CoordinateReader::new(&mut tokenizer);
        read_all(&mut reader).unwrap();
        assert_eq!(reader.coordinates(), &[]);
    }

    #[test]
    fn test_single_coord() {
        let s = "(1.2)";
        let mut tokenizer = Tokenizer::new(s);
        let mut reader = CoordinateReader::new(&mut tokenizer);
        assert!(read_all(&mut reader).is_err());
    }

    #[test]
    fn test_single_coord_pair() {
        let s = "(1.2 0.1)";
        let mut tokenizer = Tokenizer::new(s);
        let mut reader = CoordinateReader::new(&mut tokenizer);
        read_all(&mut reader).unwrap();
        assert_eq!(reader.coordinates(), &[1.2, 0.1]);
    }
}
