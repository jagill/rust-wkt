use crate::char_class::CharClass;
use crate::geometry_type::GeometryType;
use crate::parse_error::{ParseError, ParseResult};
use crate::tokenizer::{Token, Tokenizer};

pub struct WktReader<'a, S: ParseState> {
    tokens: Tokenizer<'a>,
    state: S,
}

impl<'a> WktReader<'a, Start> {
    pub fn new(wkt: &'a str) -> Self {
        Self {
            tokens: Tokenizer::new(wkt),
            state: Start {},
        }
    }

    pub fn parse(self) -> ParseResult<GeometryType> {
        let n = self.read()?;
        Ok(n.get_type())
    }

    fn read(mut self) -> ParseResult<WktReader<'a, Geometry>> {
        let Token { char_class, value } = self
            .tokens
            .next_non_whitespace()
            // this is Option<ParseResult<Token>>, so double unwrap
            .ok_or_else(ParseError::empty_wkt)??;

        if char_class != CharClass::Character {
            Err(ParseError::unexpected_initial_chars(value))
        } else {
            Ok(WktReader::<Geometry> {
                tokens: self.tokens,
                state: Geometry {
                    geometry_type: GeometryType::of(value)?,
                },
            })
        }
    }
}

pub trait ParseState {}

pub struct Start {}
impl ParseState for Start {}

pub struct Geometry {
    geometry_type: GeometryType,
}
impl ParseState for Geometry {}
impl<'a> WktReader<'a, Geometry> {
    fn get_type(&self) -> GeometryType {
        self.state.geometry_type
    }
}
