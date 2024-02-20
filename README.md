### derive-lexer

Procedural macro for regex-based sequential tokenization of input strings.

### Installation

Point to the `derive_lexer` crate in your `Cargo.toml`. Patterns require `regex`.

### Usage

Token kinds are variants of an enum that is annotated with the expected patterns.

```rust
#[derive(Tokenize, Debug)]
enum Token {
    #[lex(pattern = "^@\\w+")]
    Decorator, // Matches '@'
    #[lex(pattern = "let\\s")]
    LetBinding,
}
```

Matching is done externally, allowing custom error handling and integration with loggers, etc:

```rust
let input = "@k let n";
let mut curr_input = input;
let mut tokens = Vec::new();

while let Some((token, rest)) = Token::tokenize(curr_input) {
    tokens.push(token);
    curr_input = rest;
}

assert_eq!(tokens.len(), 2);
```

Refer to the unit tests in [`derive_lexer/src/lib.rs`](https://github.com/scaredmonad/derive-lexer/blob/main/derive_lexer/src/lib.rs).

### License

MIT License © Abdi M.
