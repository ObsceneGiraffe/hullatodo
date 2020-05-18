extern crate pest;

use super::Date;
use super::PairTag;
use super::ParseError;
use super::Todo;

use pest::Parser;

#[derive(Parser)]
#[grammar = "todo.txt.pest"]
struct TodoParser;

const ASCII_A_U8: u8 = 'A' as u8;

pub fn parse(text: &str) -> Vec<Result<Todo, ParseError>> {
    text.lines()
        .map(|line| {
            let result = TodoParser::parse(Rule::entry, line);
                
            if result.is_err() {
                // todo: error handling!!
                Err(ParseError {})
            } else {
                let entry_pair = result.unwrap().next().unwrap();
                Ok(parse_entry(entry_pair))
            }
        })
        .collect()
}

fn parse_entry(entry_pair: pest::iterators::Pair<Rule>) -> Todo {
    let mut todo: Todo = Default::default();

    for entry_inner in entry_pair.into_inner() {
        match entry_inner.as_rule() {
            Rule::complete_flag => {
                todo.is_completed = !entry_inner.as_str().is_empty();
            }
            Rule::priority_value => {
                // the parser guarantees that there is a single undercase char
                let value_char = entry_inner.as_str().chars().next().unwrap();
                todo.priority = Some(value_char as u8 - ASCII_A_U8);
            }
            Rule::date_creation => {
                todo.date_creation = as_date(entry_inner);
            }
            Rule::date_completed => {
                todo.date_completed = as_date(entry_inner);
            }
            Rule::tail => {
                todo.text = entry_inner.as_str();
                parse_tail(entry_inner, &mut todo);
            }
            _ => unreachable!()
        }
    }

    todo
}

fn parse_tail<'a>(entry_inner: pest::iterators::Pair<'a, Rule>, todo: &mut Todo<'a>) {
    for tail_inner in entry_inner.into_inner() {
        match tail_inner.as_rule() {
            Rule::context_tag => {
                todo.context_tags.push(as_tag(tail_inner));
            }
            Rule::project_tag => {
                todo.project_tags.push(as_tag(tail_inner));
            }
            Rule::pair => {
                todo.pair_tags.push(as_pair(tail_inner));
            }
            Rule::span => {}
            _ => unreachable!()
        }
    }
}

fn as_date(date_pair: pest::iterators::Pair<Rule>) -> Option<Date> {
    let mut inner = date_pair.into_inner();
    Some (
        Date { 
            year: inner.next().unwrap().as_str().parse::<u16>().unwrap(),
            month: inner.next().unwrap().as_str().parse::<u8>().unwrap(),
            day: inner.next().unwrap().as_str().parse::<u8>().unwrap()
        }
    )
}

fn as_tag(tag_pair: pest::iterators::Pair<Rule>) -> &str {
    // tags have whitespace and a single character prefixing them
    &tag_pair.as_str()[2..]
}

fn as_pair<'a>(tag_pair: pest::iterators::Pair<'a, Rule>) -> PairTag {
    let mut inner = tag_pair.into_inner();
    PairTag {
        key: inner.next().unwrap().as_str(),
        value: inner.next().unwrap().as_str()
    }
}
