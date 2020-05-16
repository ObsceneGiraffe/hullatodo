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
    fn todo_list() {
        let todos = parse(
            "this is the first todo\n\
            this is the second todo"
        );
        assert_eq!(2, todos.len());
        assert_eq!("this is the first todo", todos[0].text);
        assert_eq!("this is the second todo", todos[1].text);

        let todos = parse(
            "this is the first todo\n\
            x 2020-05-17 this is the @second todo\n\
            2020-05-18 this is the +third todo");
        
        assert_eq!(3, todos.len());
        assert_eq!("this is the first todo", todos[0].text);
        assert_eq!("this is the @second todo", todos[1].text);
        assert_eq!("this is the +third todo", todos[2].text);

        assert_eq!(false, todos[0].is_completed);
        assert_eq!(true, todos[1].is_completed);
        assert_eq!(false, todos[2].is_completed);

        assert_eq!(None, todos[0].date_completed);
        assert_eq!(Some(Date(2020, 5, 17)), todos[1].date_completed);
        assert_eq!(Some(Date(2020, 5, 18)), todos[2].date_completed);

        assert_eq!(0, todos[0].context_tags.len());
        assert_eq!(0, todos[0].project_tags.len());
        assert_eq!(1, todos[1].context_tags.len());
        assert_eq!(0, todos[1].project_tags.len());
        assert_eq!("second", todos[1].context_tags[0]);
        assert_eq!(0, todos[2].context_tags.len());
        assert_eq!(1, todos[2].project_tags.len());
        assert_eq!("third", todos[2].project_tags[0]);
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

    #[test]
    fn pairs() {
        let todos = parse("not-a-pair:this-is-just-text");
        assert_eq!(1, todos.len());
        assert_eq!("not-a-pair:this-is-just-text", todos[0].text);
        assert_eq!(0, todos[0].pair_tags.len());

        let todos = parse("message pair-key:pair-value");
        assert_eq!(1, todos.len());
        assert_eq!("message pair-key:pair-value", todos[0].text);
        assert_eq!(1, todos[0].pair_tags.len());
        assert_eq!("pair-key", todos[0].pair_tags[0].key);
        assert_eq!("pair-value", todos[0].pair_tags[0].value);

        let todos = parse("pre-message pair-key:pair-value post-message");
        assert_eq!(1, todos.len());
        assert_eq!("pre-message pair-key:pair-value post-message", todos[0].text);
        assert_eq!(1, todos[0].pair_tags.len());
        assert_eq!("pair-key", todos[0].pair_tags[0].key);
        assert_eq!("pair-value", todos[0].pair_tags[0].value);

        let todos = parse("pre-message keyA:valueA keyB:valueB post-message");
        assert_eq!("pre-message keyA:valueA keyB:valueB post-message", todos[0].text);
        assert_eq!(1, todos.len());
        assert_eq!(2, todos[0].pair_tags.len());
        assert_eq!("keyA", todos[0].pair_tags[0].key);
        assert_eq!("valueA", todos[0].pair_tags[0].value);
        assert_eq!("keyB", todos[0].pair_tags[1].key);
        assert_eq!("valueB", todos[0].pair_tags[1].value);

        // duplicate pairs should be handled by the application layer
        // it is not the parsers responsibility to destroy data
        let todos = parse("message keyA:valueA keyA:valueA");
        assert_eq!("message keyA:valueA keyA:valueA", todos[0].text);
        assert_eq!(1, todos.len());
        assert_eq!(2, todos[0].pair_tags.len());
    }

    #[test]
    fn dates() {
        let todos = parse("This is a todo without any dates");
        assert_eq!("This is a todo without any dates", todos[0].text);
        assert_eq!(None, todos[0].date_completed);
        assert_eq!(None, todos[0].date_creation);

        let todos = parse("2020-05-16 This is a todo with a completion date");
        assert_eq!("This is a todo with a completion date", todos[0].text);
        assert_eq!(Some(Date(2020, 5, 16)), todos[0].date_completed);

        let todos = parse("2020-05-16 2020-04-12 This is a todo with a completion and creation date");
        assert_eq!("This is a todo with a completion and creation date", todos[0].text);
        assert_eq!(Some(Date(2020, 5, 16)), todos[0].date_completed);
        assert_eq!(Some(Date(2020, 4, 12)), todos[0].date_creation);
    }
}

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

pub fn parse(text: &str) -> Vec<Todo> {
    text.lines()
        .filter_map(|line| {
            let result = todotxt::Parser::parse(todotxt::Rule::entry, line);
                
            if result.is_err() {
                // todo: error handling!!
                None
            } else {
                let entry_pair = result.unwrap().next().unwrap();
                Some(parse_entry(entry_pair))
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