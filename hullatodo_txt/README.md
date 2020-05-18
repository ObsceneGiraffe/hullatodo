# Hullatodo_txt

This is an implementation of a parser for the todo.txt standard.
I'm comparing two implementations, namely a PEG parser implemented with [Pest|https://docs.rs/pest/2.1.3/pest/] and parser combinator [Nom|https://docs.rs/nom/5.1.1/nom/].

The goal of this project is to create a fast todo.txt parser with friendly error messages.

# Running the tests

The two implementations have been implemented in terms of features so that they can both use the same test suite.

> :warning: Running without a feature will result in the tests failing.

To run the Pest parser version:

```zsh
cargo test --features pest_parser
```

To run the Nom parser version:

```zsh
cargo test --features nom_parser
```