pub use derive_lexer_macro;

pub trait Tokenize {
    fn tokenize(input: &str) -> Option<(Self, &str)>
    where
        Self: Sized;
}
