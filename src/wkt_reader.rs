use crate::char_class::CharClass;
use crate::geometry_type::GeometryType;
use crate::tokenizer::Tokenizer;

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

    pub fn parse(self) -> Result<GeometryType, String> {
        let n = self.read()?;
        Ok(n.get_type())
    }

    fn read(mut self) -> Result<WktReader<'a, Geometry>, String> {
        let (char_class, token) = self
            .tokens
            .next_non_whitespace()
            .ok_or_else(|| String::from("Empty WKT"))?;
        if char_class != CharClass::Character {
            Err(format!("Unexpected initial characters: '{}'", token))
        } else {
            Ok(WktReader::<Geometry> {
                tokens: self.tokens,
                state: Geometry {
                    geometry_type: GeometryType::of(token)?,
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
