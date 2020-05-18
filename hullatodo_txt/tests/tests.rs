extern crate hullatodo_txt;

#[cfg(test)]
mod tests {
    use super::*;
    use hullatodo_txt::Date;

    #[test]
    fn empty() {
        let todos = hullatodo_txt::parse("");
        assert_eq!(0, todos.len());
    }

    #[test]
    fn simplest() {
        let todos = hullatodo_txt::parse("this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", first.text);
        assert_eq!(false, first.is_completed);
    }

    #[test]
    fn todo_list() {
        let todos = hullatodo_txt::parse(
            "this is the first todo\n\
            this is the second todo"
        );
        assert_eq!(2, todos.len());
        let first = todos[0].as_ref().unwrap();
        let second = todos[1].as_ref().unwrap();
        assert_eq!("this is the first todo", first.text);
        assert_eq!("this is the second todo", second.text);

        let todos = hullatodo_txt::parse(
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
        assert_eq!(Some(Date { year: 2020, month: 5, day: 17 }), second.date_completed);
        assert_eq!(Some(Date { year: 2020, month: 5, day: 18 }), third.date_completed);

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
        let todos = hullatodo_txt::parse("x this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("this is a todo", first.text);
        assert_eq!(true, first.is_completed);

        let todos = hullatodo_txt::parse("xylophone lesson");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("xylophone lesson", first.text);
        assert_eq!(false, first.is_completed);

        let todos = hullatodo_txt::parse("X 2012-01-01 Make resolutions");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("X 2012-01-01 Make resolutions", first.text);
        assert_eq!(false, first.is_completed);

        let todos = hullatodo_txt::parse("(A) x Find ticket prices");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("x Find ticket prices", first.text);
        assert_eq!(false, first.is_completed);
    }

    #[test]
    fn priority() {
        let todos = hullatodo_txt::parse("(A) this has the highest priority");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("this has the highest priority", first.text);
        assert_eq!(Some(0), first.priority);

        let todos = hullatodo_txt::parse("(Z) this has the lowest priority");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("this has the lowest priority", first.text);
        assert_eq!(Some(25), first.priority);

        let todos = hullatodo_txt::parse("(+) this is a todo");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(+) this is a todo", first.text);
        assert_eq!(None, first.priority);

        let todos = hullatodo_txt::parse("Really gotta call Mom (A) @phone @someday");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Really gotta call Mom (A) @phone @someday", first.text);
        assert_eq!(None, first.priority);

        let todos = hullatodo_txt::parse("(b) Get back to the boss");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(b) Get back to the boss", first.text);
        assert_eq!(None, first.priority);

        let todos = hullatodo_txt::parse("(B)->Submit TPS report");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("(B)->Submit TPS report", first.text);
        assert_eq!(None, first.priority);
    }

    #[test]
    fn tags() {
        let todos = hullatodo_txt::parse("(A) Call Mom +Family +PeaceLoveAndHappiness @iphone @phone");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Call Mom +Family +PeaceLoveAndHappiness @iphone @phone", first.text);
        assert_eq!(Some(0), first.priority);
        assert_eq!(2, first.context_tags.len());
        assert_eq!("Family", first.project_tags[0]);
        assert_eq!("PeaceLoveAndHappiness", first.project_tags[1]);
        assert_eq!("iphone", first.context_tags[0]);
        assert_eq!("phone", first.context_tags[1]);

        let todos = hullatodo_txt::parse("Email SoAndSo at soandso@example.com");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Email SoAndSo at soandso@example.com", first.text);
        assert_eq!(0, first.context_tags.len());

        let todos = hullatodo_txt::parse("Learn how to add 2+2");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Learn how to add 2+2", first.text);
        assert_eq!(0, first.project_tags.len());

        let todos = hullatodo_txt::parse("Do Unicode tags work @ハラトド do they?");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("Do Unicode tags work @ハラトド do they?", first.text);
        assert_eq!(1, first.context_tags.len());
        assert_eq!("ハラトド", first.context_tags[0]);
    }

    #[test]
    fn pairs() {
        let todos = hullatodo_txt::parse("not-a-pair:this-is-just-text");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("not-a-pair:this-is-just-text", first.text);
        assert_eq!(0, first.pair_tags.len());

        let todos = hullatodo_txt::parse("message pair-key:pair-value");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("message pair-key:pair-value", first.text);
        assert_eq!(1, first.pair_tags.len());
        assert_eq!("pair-key", first.pair_tags[0].key);
        assert_eq!("pair-value", first.pair_tags[0].value);

        let todos = hullatodo_txt::parse("pre-message pair-key:pair-value post-message");
        let first = todos[0].as_ref().unwrap();
        assert_eq!(1, todos.len());
        assert_eq!("pre-message pair-key:pair-value post-message", first.text);
        assert_eq!(1, first.pair_tags.len());
        assert_eq!("pair-key", first.pair_tags[0].key);
        assert_eq!("pair-value", first.pair_tags[0].value);

        let todos = hullatodo_txt::parse("pre-message keyA:valueA keyB:valueB post-message");
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
        let todos = hullatodo_txt::parse("message keyA:valueA keyA:valueA");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("message keyA:valueA keyA:valueA", first.text);
        assert_eq!(1, todos.len());
        assert_eq!(2, first.pair_tags.len());
    }

    #[test]
    fn dates() {
        let todos = hullatodo_txt::parse("This is a todo without any dates");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo without any dates", first.text);
        assert_eq!(None, first.date_completed);
        assert_eq!(None, first.date_creation);

        let todos = hullatodo_txt::parse("2020-05-16 This is a todo with a completion date");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo with a completion date", first.text);
        assert_eq!(Some(Date { year: 2020, month: 5, day: 16 }), first.date_completed);

        let todos = hullatodo_txt::parse("2020-05-16 2020-04-12 This is a todo with a completion and creation date");
        let first = todos[0].as_ref().unwrap();
        assert_eq!("This is a todo with a completion and creation date", first.text);
        assert_eq!(Some(Date { year: 2020, month: 5, day: 16 }), first.date_completed);
        assert_eq!(Some(Date { year: 2020, month: 4, day: 12 }), first.date_creation);
    }
}