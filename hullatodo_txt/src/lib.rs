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
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", todos[0].text);
        assert_eq!(false, todos[0].is_completed);
    }

    #[test]
    fn completeness() {
        let todos = parse("x this is a todo");
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", todos[0].text);
        assert_eq!(true, todos[0].is_completed);

        let todos = parse("xylophone lesson");
        assert_eq!(1, todos.len());
        assert_eq!("xylophone lesson", todos[0].text);
        assert_eq!(false, todos[0].is_completed);

        let todos = parse("X 2012-01-01 Make resolutions");
        assert_eq!(1, todos.len());
        assert_eq!("X 2012-01-01 Make resolutions", todos[0].text);
        assert_eq!(false, todos[0].is_completed);

        let todos = parse("(A) x Find ticket prices");
        assert_eq!(1, todos.len());
        assert_eq!("x Find ticket prices", todos[0].text);
        assert_eq!(false, todos[0].is_completed);
    }

    #[test]
    fn priority() {
        let todos = parse("(A) this has the highest priority");
        assert_eq!("this has the highest priority", todos[0].text);
        assert_eq!(Some(0), todos[0].priority);

        let todos = parse("(Z) this has the lowest priority");
        assert_eq!("this has the lowest priority", todos[0].text);
        assert_eq!(Some(25), todos[0].priority);

        let todos = parse("(+) this is a todo");
        assert_eq!("(+) this is a todo", todos[0].text);
        assert_eq!(None, todos[0].priority);

        let todos = parse("Really gotta call Mom (A) @phone @someday");
        assert_eq!("Really gotta call Mom (A) @phone @someday", todos[0].text);
        assert_eq!(None, todos[0].priority);

        let todos = parse("(b) Get back to the boss");
        assert_eq!("(b) Get back to the boss", todos[0].text);
        assert_eq!(None, todos[0].priority);

        let todos = parse("(B)->Submit TPS report");
        assert_eq!("(B)->Submit TPS report", todos[0].text);
        assert_eq!(None, todos[0].priority);
    }
}

use pest::Parser;

#[derive(Parser)]
#[grammar = "todo.txt.pest"]
pub struct TodoTxtParser;

#[derive(Debug, Default)]
pub struct Date(u16, u8, u8);

#[derive(Default)]
pub struct Todo<'a> {
    pub is_completed: bool,
    pub priority: Option<u8>,
    pub date_creation: Date,
    pub date_completed: Date,
    pub text: &'a str,
    pub contexts: Vec<&'a str>,
    pub projects: Vec<&'a str>,
    pub context_tags: Vec<&'a str>,
    pub kv_tags: Vec<(&'a str, &'a str)>
}

pub fn parse(text: &str) -> Vec<Todo> {
    let entry_list = TodoTxtParser::parse(Rule::entry_list, text)
        .expect("unsuccessful parse").next().unwrap();

    println!("parse!");

    let result = entry_list.into_inner()
        .filter_map(|pair| {
            println!("{:?}", pair);

            match pair.as_rule() {
                Rule::entry => parse_entry(pair),
                Rule::EOI => None,
                _ => unreachable!()
            }
        }).collect();

    return result;
}

fn parse_entry(entry: pest::iterators::Pair<Rule>) -> Option<Todo> {
    let mut todo: Todo = Default::default();
    for field in entry.into_inner() {
        match field.as_rule() {
            Rule::complete_flag => {
                todo.is_completed = !field.as_str().is_empty();
            }
            Rule::priority_value => {
                let value_char = field.as_str().chars().next().unwrap();
                if value_char.is_ascii_uppercase() {
                    todo.priority = Some(value_char as u8 - 'A' as u8);
                }
            }
            Rule::date_creation => {
                println!("{:?}", field.as_str());
                todo.date_creation = Default::default();
            }
            Rule::date_completed => {
                println!("{:?}", field.as_str());
                todo.date_completed = Default::default();
            }
            Rule::tail => {
                todo.text = field.as_str();
            }
            _ => unreachable!()
        }
    }

    if todo.text.is_empty() { None } else { Some(todo) }
}
