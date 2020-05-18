#[macro_use]
extern crate pest_derive;

use std::error;
use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

#[derive(Debug, PartialEq)]
pub struct PairTag<'a> {
    pub key: &'a str,
    pub value: &'a str
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Default)]
pub struct Todo<'a> {
    pub is_completed: bool,
    pub priority: Option<u8>,
    pub date_creation: Option<Date>,
    pub date_completed: Option<Date>,
    pub text: &'a str,
    pub context_tags: Vec<&'a str>,
    pub project_tags: Vec<&'a str>,
    pub pair_tags: Vec<PairTag<'a>>
}

#[cfg(not(feature = "nom_parser"))]
mod pest_parser;

#[cfg(not(feature = "nom_parser"))]
pub fn parse(text: &str) -> Vec<Result<Todo, ParseError>> {
    pest_parser::parse(text)
}

#[cfg(feature = "nom_parser")]
mod nom_parser;

#[cfg(feature = "nom_parser")]
pub fn parse(text: &str) -> Vec<Result<Todo, ParseError>> {
    nom_parser::parse(text)
}