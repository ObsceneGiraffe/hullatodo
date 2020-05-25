extern crate nom;

use super::{
    Date,
    ParseWarning,
    Todo,
    TodoLines
};

use nom::{
    IResult,
    bytes::complete::{is_not, take, take_until, take_while},
    character::{is_digit},
    character::complete::{anychar, char, newline},
    combinator::{flat_map, map, map_res, opt, verify},
    error::{context, ErrorKind, ParseError, VerboseError},
    sequence::{delimited, tuple},
};

// use std::{
//     vec::IntoIter,
//     iter::IntoIterator
// };

#[cfg(test)]
mod test {
    use super::*;

    use nom::{
        error::{ErrorKind, VerboseErrorKind}
    };

    #[test]
    fn test_not_whitespace() {
        assert_eq!(not_whitespace("abcd efg"), Ok((" efg", "abcd")));
		assert_eq!(not_whitespace("abcd\tefg"), Ok(("\tefg", "abcd")));
		assert_eq!(not_whitespace(" abcdefg"), Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot))));
    }

    #[test]
    fn test_complete() {
        // let parser = complete::<(&str, ErrorKind)>;
        let parser = complete::<VerboseError<&str>>;
        assert_eq!(parser("X hello"), Ok((" hello", (true, vec![]))));
        assert_eq!(parser("x hello"), Ok(("x hello", (false, vec![]))));
        assert_eq!(parser("hello"), Ok(("hello", (false, vec![]))));
    }

    #[test]
    fn test_priority() {

        let parser = priority::<(&str, ErrorKind)>;
        // let parser = priority::<VerboseError<&str>>;
        assert_eq!(parser(""), Ok(("", (None, vec![]))));
        assert_eq!(parser("(A) hello"), Ok((" hello", (Some(0u8), vec![]))));
        assert_eq!(parser("(Z) hello"), Ok((" hello", (Some(25u8), vec![]))));
        
        println!("res: {:#?}", parser("(AA) hello"));
        // assert_eq!(parser("(AA) hello"), 
        //     Err(
        //         nom::Err::Error(
        //             VerboseError { 
        //                 errors: vec![
        //                     ("A) hello", VerboseErrorKind::Char(')')), 
        //                     ("(AA) hello", VerboseErrorKind::Context("priority"))
        //                 ] 
        //             }
        //         )
        //     )
        // );

        // assert_eq!(parser("(a) hello"), 
        //     Err(
        //         nom::Err::Error(
        //             VerboseError { 
        //                 errors: vec![
        //                     ("a) hello", VerboseErrorKind::Nom(ErrorKind::Verify)), 
        //                     ("(a) hello", VerboseErrorKind::Context("priority"))
        //                 ] 
        //             }
        //         )
        //     )
        // );
    }

    #[test]
    fn test_date() {
        let parser = date::<(&str, ErrorKind)>;
        // let parser = date::<VerboseError<&str>>;
        // TodoParserResult<Option<Date>, E>
        assert_eq!(parser("2020-05-23 hello"), 
            Ok((
                " hello",
                (
                    Some(Date {year: 2020, month: 05, day: 23}),
                    vec![]
                )
            ))
        );

        // assert_eq!(parser("2020-05 hello"), 
        //     Err(
        //         nom::Err::Error(
        //             VerboseError { 
        //                 errors: vec![
        //                     ("a) hello", VerboseErrorKind::Nom(ErrorKind::Verify)), 
        //                     ("(a) hello", VerboseErrorKind::Context("priority"))
        //                 ] 
        //             }
        //         )
        //     )
        // );
    }
}

const ASCII_A_U8: u8 = 'A' as u8;
const DATE_DELIMITER: char = '-';

// The TodoParser defines a custom Result and a custom error 
// so that parse warnings can be passes along in each case
// these warnings buble up to the top of the call graph and are aggregated together 
#[derive(Debug, PartialEq)]
struct TodoParserError<'a>(&'a str, ErrorKind, Vec<ParseWarning>);

impl <'a> ParseError<&'a str> for TodoParserError<'a> {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        TodoParserError(input, kind, vec![])
    }
  
    fn append(_: &'a str, _: ErrorKind, other: Self) -> Self {
        other
    }
}

type TodoParserResult<'a, T, E = TodoParserError<'a>> = IResult<&'a str, (T, Vec<ParseWarning>), E>;

pub fn parse<'a>(text: &'a str) -> TodoLines<'a> {
    text.lines()
        .map(|line| root::<TodoParserError>(line))
        .collect()
}

fn root<'a, E: ParseError<&'a str>>(i: &'a str) -> Option<Todo> {
    let parser = tuple::<_, _, TodoParserError<'a>, _>((
        complete,
        priority,
        date,
        date,
        tail
    ));

    match parser(i) {
        Ok((_, (complete_pair, priority_pair, date_completed_pair, date_creation_pair, tail_pair))) => {
             
            let (is_completed, complete_warnings) = complete_pair;
            let (priority, priority_warnings) = priority_pair;
            let (date_completed, date_comleted_warnings) = date_completed_pair;
            let (date_creation, date_created_warnings) = date_creation_pair;
            let (tail, tail_warnings) = tail_pair;

            let ready = vec![complete_warnings, priority_warnings, date_comleted_warnings, date_created_warnings, tail_warnings];
            let warnings = ready.into_iter().fold(vec![],
                |mut a, b| { 
                    a.extend(b); 
                    a 
                }
            );

            Some(Todo {
                is_completed: is_completed,
                priority: priority,
                date_completed: date_completed,
                date_creation: date_creation,
                text: tail,
                context_tags: vec![],
                project_tags: vec![],
                pair_tags: vec![],
                warnings: warnings
            })
        },
        Err(_) => {
            None
        }
    }
}

// pub fn concat<I>(iterable: I) -> I::Item
//     where I: IntoIterator,
//           I::Item: Extend<<<I as IntoIterator>::Item as IntoIterator>::Item> + IntoIterator + Default
// {
//     iterable.into_iter().fold1(|mut a, b| { a.extend(b); a }).unwrap_or_else(|| <_>::default())
// }

#[allow(dead_code)]
fn not_whitespace(input: &str) -> IResult<&str, &str> {
    is_not(" \t")(input)
}

fn wp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn complete<'a, E: ParseError<&'a str>>(input: &'a str) -> TodoParserResult<bool, E> {
    match opt(char('X'))(input) {
        Ok((line, Some(_))) => Ok((line, (true, vec![]))),
        Ok((line, None)) => Ok((line, (false, vec![]))),
        Err(x) => Err(x)
    }
}

fn is_upper(input: &char) -> bool {
    input.is_uppercase()
}

#[allow(dead_code)]
fn priority<'a, E: ParseError<&'a str>>(i: &'a str) -> TodoParserResult<Option<u8>, E> {
    let parser = context(
        "priority",
        map_res(
            delimited(
                char('('),
                verify(anychar, is_upper),
                char(')')
            ),
            |c: char| -> Result<u8, E> { 
                Ok((c as u8) - ASCII_A_U8)
            }
        )
    );
    
    parser(i)
        .map(|(i, prio)| (i, (Some(prio), vec![])))
        .or_else(|err: nom::Err<E>| {
            match err {
                nom::Err::Error(e) => {
                    println!("err: {:?}", e);
                    Ok((i, (None, vec![])))
                },
                nom::Err::Failure(_e) => Ok((i, (None, vec![]))),
                nom::Err::Incomplete(_) => Ok((i, (None, vec![])))
            }
        })
}

fn all_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

fn take_digits<'a, E: ParseError<&'a str>>(n: usize) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    move |i: &'a str| {
        verify(take(n), |s: &str| all_digits(s))(i)
    }
}

fn date<'a, E: ParseError<&'a str>>(i: &'a str) -> TodoParserResult<Option<Date>, E> {
let year = take_digits(4usize);
    let month = take_digits(2usize);
    let day = take_digits(2usize);
    let parser = map(
        tuple((year, char(DATE_DELIMITER), month, char(DATE_DELIMITER), day)),
        |(year, _, month, _, day)| {
            Date {
                year: year.parse::<u16>().unwrap(),
                month: month.parse::<u8>().unwrap(),
                day: day.parse::<u8>().unwrap()
            }
        }
    );

    parser(i)
        .map(|(i, date)| (i, (Some(date), vec![])))
        .or_else(|err: nom::Err<E>| {
            Err(err)
        })
}

fn tail<'a, E: ParseError<&'a str>>(i: &'a str) -> TodoParserResult<&'a str, E> {
    take_until("\n")(i)
        .map(|(i, s)| (i, (s, vec![])))
        .or_else(|err: nom::Err<E>| {
            Err(err)
        })
}