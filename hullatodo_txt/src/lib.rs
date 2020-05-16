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

    #[test]
    fn tags() {
        let todos = parse("(A) Call Mom +Family +PeaceLoveAndHappiness @iphone @phone");
        assert_eq!("Call Mom +Family +PeaceLoveAndHappiness @iphone @phone", todos[0].text);
        assert_eq!(Some(0), todos[0].priority);
        assert_eq!(2, todos[0].context_tags.len());
        assert_eq!("Family", todos[0].project_tags[0]);
        assert_eq!("PeaceLoveAndHappiness", todos[0].project_tags[1]);
        assert_eq!("iphone", todos[0].context_tags[0]);
        assert_eq!("phone", todos[0].context_tags[1]);

        let todos = parse("Email SoAndSo at soandso@example.com");
        assert_eq!("Email SoAndSo at soandso@example.com", todos[0].text);
        assert_eq!(0, todos[0].context_tags.len());

        let todos = parse("Learn how to add 2+2");
        assert_eq!("Learn how to add 2+2", todos[0].text);
        assert_eq!(0, todos[0].project_tags.len());

        let todos = parse("Do Unicode tags work @ハラトド do they?");
        assert_eq!("Do Unicode tags work @ハラトド do they?", todos[0].text);
        assert_eq!(1, todos[0].context_tags.len());
        assert_eq!("ハラトド", todos[0].context_tags[0]);
    }
}

use pest::Parser;

mod todotxt {
    #[derive(Parser)]
    #[grammar = "todo.txt.pest"]
    pub struct Parser;
}

#[derive(Debug, Default)]
pub struct Date(u16, u8, u8);

#[derive(Debug)]
pub struct PairTag<'a>(&'a str, &'a str);

#[derive(Default)]
pub struct Todo<'a> {
    pub is_completed: bool,
    pub priority: Option<u8>,
    pub date_creation: Date,
    pub date_completed: Date,
    pub text: &'a str,
    pub context_tags: Vec<&'a str>,
    pub project_tags: Vec<&'a str>,
    pub pair_tags: Vec<PairTag<'a>>
}

pub fn parse(text: &str) -> Vec<Todo> {
    let entry_list = todotxt::Parser::parse(todotxt::Rule::entry_list, text)
        .expect("unsuccessful parse").next().unwrap();

    println!("parse!");

    let result = entry_list.into_inner()
        .filter_map(|pair| {
            println!("{:?}", pair);
            parse_entry(pair)
        }).collect();

    return result;
}

fn parse_entry(entry: pest::iterators::Pair<todotxt::Rule>) -> Option<Todo> {
    let mut todo: Todo = Default::default();
    for entry_inner in entry.into_inner() {
        match entry_inner.as_rule() {
            todotxt::Rule::complete_flag => {
                todo.is_completed = !entry_inner.as_str().is_empty();
            }
            todotxt::Rule::priority_value => {
                let value_char = entry_inner.as_str().chars().next().unwrap();
                if value_char.is_ascii_uppercase() {
                    todo.priority = Some(value_char as u8 - 'A' as u8);
                }
            }
            todotxt::Rule::date_creation => {
                println!("{:?}", entry_inner.as_str());
                todo.date_creation = Default::default();
            }
            todotxt::Rule::date_completed => {
                println!("{:?}", entry_inner.as_str());
                todo.date_completed = Default::default();
            }
            todotxt::Rule::tail => {
                todo.text = entry_inner.as_str();

                for tail_inner in entry_inner.into_inner() {
                    match tail_inner.as_rule() {
                        todotxt::Rule::context_tag => {
                            // tags whitespace and a single character prefixing them
                            let tag = &tail_inner.as_str()[2..];
                            todo.context_tags.push(tag);
                        }
                        todotxt::Rule::project_tag => {
                            let tag = &tail_inner.as_str()[2..];
                            todo.project_tags.push(tag);
                        }
                        todotxt::Rule::span => {}
                        _ => unreachable!()
                    }
                }
            }
            _ => unreachable!()
        }
    }

    if todo.text.is_empty() { None } else { Some(todo) }
}
