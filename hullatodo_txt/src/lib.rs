extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let todos = parse("");
        assert_eq!(0, todos.len());
    }

    #[test]
    fn simplest() {
        let todos = parse("this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", first.text);
        assert_eq!(false, first.is_completed);
    }

    #[test]
    fn todo_list() {
        let todos = parse(
            "this is the first todo\n\
            this is the second todo"
        );
        assert_eq!(2, todos.len());
        let first = todos[0].as_ref().unwrap();
        let second = todos[1].as_ref().unwrap();
        assert_eq!("this is the first todo", first.text);
        assert_eq!("this is the second todo", second.text);

        let todos = parse(
            "this is the first todo\n\
            x 2020-05-17 this is the @second todo\n\
            2020-05-18 this is the +third todo");
        
        assert_eq!(3, todos.len());
        let first = todos[0].as_ref().unwrap();
        let second = todos[1].as_ref().unwrap();
        let third = todos[2].as_ref().unwrap();
        assert_eq!("this is the first todo", first.text);
        assert_eq!("this is the @second todo", second.text);
        assert_eq!("this is the +third todo", third.text);

        assert_eq!(false, first.is_completed);
        assert_eq!(true, second.is_completed);
        assert_eq!(false, third.is_completed);

        assert_eq!(None, first.date_completed);
        assert_eq!(Some(Date(2020, 5, 17)), second.date_completed);
        assert_eq!(Some(Date(2020, 5, 18)), third.date_completed);

        assert_eq!(0, first.context_tags.len());
        assert_eq!(0, first.project_tags.len());
        assert_eq!(1, second.context_tags.len());
        assert_eq!(0, second.project_tags.len());
        assert_eq!("second", second.context_tags[0]);
        assert_eq!(0, third.context_tags.len());
        assert_eq!(1, third.project_tags.len());
        assert_eq!("third", third.project_tags[0]);
    }

    #[test]
    fn completeness() {
        let todos = parse("x this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", first.text);
        assert_eq!(true, first.is_completed);

        let todos = parse("xylophone lesson");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("xylophone lesson", first.text);
        assert_eq!(false, first.is_completed);

        let todos = parse("X 2012-01-01 Make resolutions");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("X 2012-01-01 Make resolutions", first.text);
        assert_eq!(false, first.is_completed);

        let todos = parse("(A) x Find ticket prices");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("x Find ticket prices", first.text);
        assert_eq!(false, first.is_completed);
    }

    #[test]
    fn priority() {
        let todos = parse("(A) this has the highest priority");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("this has the highest priority", first.text);
        assert_eq!(Some(0), first.priority);

        let todos = parse("(Z) this has the lowest priority");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("this has the lowest priority", first.text);
        assert_eq!(Some(25), first.priority);

        let todos = parse("(+) this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(+) this is a todo", first.text);
        assert_eq!(None, first.priority);

        let todos = parse("Really gotta call Mom (A) @phone @someday");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Really gotta call Mom (A) @phone @someday", first.text);
        assert_eq!(None, first.priority);

        let todos = parse("(b) Get back to the boss");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(b) Get back to the boss", first.text);
        assert_eq!(None, first.priority);

        let todos = parse("(B)->Submit TPS report");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(B)->Submit TPS report", first.text);
        assert_eq!(None, first.priority);
    }

    #[test]
    fn tags() {
        let todos = parse("(A) Call Mom +Family +PeaceLoveAndHappiness @iphone @phone");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Call Mom +Family +PeaceLoveAndHappiness @iphone @phone", first.text);
        assert_eq!(Some(0), first.priority);
        assert_eq!(2, first.context_tags.len());
        assert_eq!("Family", first.project_tags[0]);
        assert_eq!("PeaceLoveAndHappiness", first.project_tags[1]);
        assert_eq!("iphone", first.context_tags[0]);
        assert_eq!("phone", first.context_tags[1]);

        let todos = parse("Email SoAndSo at soandso@example.com");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Email SoAndSo at soandso@example.com", first.text);
        assert_eq!(0, first.context_tags.len());

        let todos = parse("Learn how to add 2+2");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Learn how to add 2+2", first.text);
        assert_eq!(0, first.project_tags.len());

        let todos = parse("Do Unicode tags work @ハラトド do they?");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Do Unicode tags work @ハラトド do they?", first.text);
        assert_eq!(1, first.context_tags.len());
        assert_eq!("ハラトド", first.context_tags[0]);
    }

    #[test]
    fn pairs() {
        let todos = parse("not-a-pair:this-is-just-text");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("not-a-pair:this-is-just-text", first.text);
        assert_eq!(0, first.pair_tags.len());

        let todos = parse("message pair-key:pair-value");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("message pair-key:pair-value", first.text);
        assert_eq!(1, first.pair_tags.len());
        assert_eq!("pair-key", first.pair_tags[0].key);
        assert_eq!("pair-value", first.pair_tags[0].value);

        let todos = parse("pre-message pair-key:pair-value post-message");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("pre-message pair-key:pair-value post-message", first.text);
        assert_eq!(1, first.pair_tags.len());
        assert_eq!("pair-key", first.pair_tags[0].key);
        assert_eq!("pair-value", first.pair_tags[0].value);

        let todos = parse("pre-message keyA:valueA keyB:valueB post-message");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("pre-message keyA:valueA keyB:valueB post-message", first.text);
        assert_eq!(1, todos.len());
        assert_eq!(2, first.pair_tags.len());
        assert_eq!("keyA", first.pair_tags[0].key);
        assert_eq!("valueA", first.pair_tags[0].value);
        assert_eq!("keyB", first.pair_tags[1].key);
        assert_eq!("valueB", first.pair_tags[1].value);

        // duplicate pairs should be handled by the application layer
        // it is not the parsers responsibility to destroy data
        let todos = parse("message keyA:valueA keyA:valueA");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("message keyA:valueA keyA:valueA", first.text);
        assert_eq!(1, todos.len());
        assert_eq!(2, first.pair_tags.len());
    }

    #[test]
    fn dates() {
        let todos = parse("This is a todo without any dates");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo without any dates", first.text);
        assert_eq!(None, first.date_completed);
        assert_eq!(None, first.date_creation);

        let todos = parse("2020-05-16 This is a todo with a completion date");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo with a completion date", first.text);
        assert_eq!(Some(Date(2020, 5, 16)), first.date_completed);

        let todos = parse("2020-05-16 2020-04-12 This is a todo with a completion and creation date");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo with a completion and creation date", first.text);
        assert_eq!(Some(Date(2020, 5, 16)), first.date_completed);
        assert_eq!(Some(Date(2020, 4, 12)), first.date_creation);
    }
}

use std::error;
use std::fmt;

use pest::Parser;

const ASCII_A_U8: u8 = 'A' as u8;

mod todotxt {
    #[derive(Parser)]
    #[grammar = "todo.txt.pest"]
    pub struct Parser;
}

#[derive(Debug, Default, PartialEq)]
pub struct Date(u16, u8, u8);

#[derive(Debug, PartialEq)]
pub struct PairTag<'a> {
    key: &'a str,
    value: &'a str
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

pub fn parse(text: &str) -> Vec<Result<Todo, ParseError>> {
    text.lines()
        .map(|line| {
            let result = todotxt::Parser::parse(todotxt::Rule::entry, line);
                
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

fn parse_entry(entry_pair: pest::iterators::Pair<todotxt::Rule>) -> Todo {
    let mut todo: Todo = Default::default();

    for entry_inner in entry_pair.into_inner() {
        match entry_inner.as_rule() {
            todotxt::Rule::complete_flag => {
                todo.is_completed = !entry_inner.as_str().is_empty();
            }
            todotxt::Rule::priority_value => {
                // the parser guarantees that there is a single undercase char
                let value_char = entry_inner.as_str().chars().next().unwrap();
                todo.priority = Some(value_char as u8 - ASCII_A_U8);
            }
            todotxt::Rule::date_creation => {
                todo.date_creation = as_date(entry_inner);
            }
            todotxt::Rule::date_completed => {
                todo.date_completed = as_date(entry_inner);
            }
            todotxt::Rule::tail => {
                todo.text = entry_inner.as_str();
                parse_tail(entry_inner, &mut todo);
            }
            _ => unreachable!()
        }
    }

    todo
}

fn parse_tail<'a>(entry_inner: pest::iterators::Pair<'a, todotxt::Rule>, todo: &mut Todo<'a>) {
    for tail_inner in entry_inner.into_inner() {
        match tail_inner.as_rule() {
            todotxt::Rule::context_tag => {
                todo.context_tags.push(as_tag(tail_inner));
            }
            todotxt::Rule::project_tag => {
                todo.project_tags.push(as_tag(tail_inner));
            }
            todotxt::Rule::pair => {
                todo.pair_tags.push(as_pair(tail_inner));
            }
            todotxt::Rule::span => {}
            _ => unreachable!()
        }
    }
}

fn as_date(date_pair: pest::iterators::Pair<todotxt::Rule>) -> Option<Date> {
    let mut inner = date_pair.into_inner();
    let year = inner.next().unwrap().as_str().parse::<u16>().unwrap();
    let month = inner.next().unwrap().as_str().parse::<u8>().unwrap();
    let day = inner.next().unwrap().as_str().parse::<u8>().unwrap();
    Some(Date(year, month, day))
}

fn as_tag(tag_pair: pest::iterators::Pair<todotxt::Rule>) -> &str {
    // tags have whitespace and a single character prefixing them
    &tag_pair.as_str()[2..]
}

fn as_pair<'a>(tag_pair: pest::iterators::Pair<'a, todotxt::Rule>) -> PairTag {
    let mut inner = tag_pair.into_inner();
    PairTag {
        key: inner.next().unwrap().as_str(),
        value: inner.next().unwrap().as_str()
    }
}