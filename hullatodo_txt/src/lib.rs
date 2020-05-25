#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[cfg_attr(not(feature = "nom_parser"), macro_use)]
extern crate pest_derive;

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

#[derive(Debug, Clone, PartialEq)]
pub struct ParseWarning {
    pub text_span: (u32, u32),
    pub char: Option<u32>,
    pub kind: ParseWarningKind
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseWarningKind {
    LowerCaseCompleteFlag,          // x hello
    ManyCompleteFlags,              // XX hello
    MalformedDate(DateKind),        // 201-05-23 hello
    TooManyDates,                   // 2001-05-23 2001-05-23 2001-05-23 hello
    OrphanContextTag,               // hello @ hello
    ContextTagPrefixInTag,          // hello @tag@content
    OrphanProjectTag,               // hello # and hello
    ProjectTagPrefixInTag,          // hello #pro#ect# hello
    OrphanPairTagKey                // hello key: hello
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateKind {
    Creation,
    Completed
}

impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub type TodoLines<'a> = Vec<Option<Todo<'a>>>;

#[derive(Default)]
pub struct Todo<'a> {
    pub is_completed: bool,
    pub priority: Option<u8>,
    pub date_completed: Option<Date>,
    pub date_creation: Option<Date>,
    pub text: &'a str,
    pub context_tags: Vec<&'a str>,
    pub project_tags: Vec<&'a str>,
    pub pair_tags: Vec<PairTag<'a>>,
    pub warnings: Vec<ParseWarning>
}

#[cfg(not(feature = "nom_parser"))]
mod pest_parser;

#[cfg(not(feature = "nom_parser"))]
pub fn parse<'a>(text: &'a str) -> TodoLines<'a> {
    pest_parser::parse(text)
}

#[cfg(feature = "nom_parser")]
mod nom_parser;

#[cfg(feature = "nom_parser")]
pub fn parse(text: &'_ str) -> TodoLines<'_> {
    nom_parser::parse(text)
}