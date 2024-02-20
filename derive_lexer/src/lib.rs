pub use derive_lexer_macro;

pub trait Tokenize {
    fn tokenize(input: &str) -> Option<(Self, &str)>
    where
        Self: Sized;
}

mod test {
    use crate::derive_lexer_macro::Tokenize;
    use crate::Tokenize;

    #[derive(Tokenize, Debug)]
    enum Token {
        #[lex(pattern = "@")]
        Decorator,
        #[lex(pattern = "let")]
        LetBinding,
    }

    #[test]
    fn can_derive_tokens() {

    }
}
