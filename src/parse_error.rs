use std::error;
use std::fmt;

use crate::tokenizer::Token;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub error_type: ParseErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorType {
    InvalidToken(usize, String),
    InvalidCharClass(char),
    UnknownGeometryType(String),
    UnexpectedInitialChars(String),
    EmptyWkt,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing wkt string. ")?;
        match &self.error_type {
            ParseErrorType::InvalidCharClass(c) => write!(f, "Invalid character '{}'", c),
            ParseErrorType::UnknownGeometryType(s) => write!(f, "Unknown geometry type: {}", s),
            ParseErrorType::UnexpectedInitialChars(s) => {
                write!(f, "Unexpected initial characters: '{}'", s)
            }
            ParseErrorType::EmptyWkt => write!(f, "Empty WKT"),
            ParseErrorType::InvalidToken(i, s) => {
                write!(f, "Invalid token at index {}: '{}'", i, s)
            }
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl ParseError {
    pub fn invalid_char_class(c: char) -> ParseError {
        ParseError {
            error_type: ParseErrorType::InvalidCharClass(c),
        }
    }

    pub fn unknown_geometry_type(s: &str) -> ParseError {
        ParseError {
            error_type: ParseErrorType::UnknownGeometryType(String::from(s)),
        }
    }

    pub fn unexpected_initial_chars(s: &str) -> ParseError {
        ParseError {
            error_type: ParseErrorType::UnexpectedInitialChars(String::from(s)),
        }
    }

    pub fn empty_wkt() -> ParseError {
        ParseError {
            error_type: ParseErrorType::EmptyWkt,
        }
    }

    pub fn invalid_token(token: &Token) -> ParseError {
        ParseError {
            error_type: ParseErrorType::InvalidToken {
                0: token.index,
                1: String::from(token.value),
            },
        }
    }
}
