pub use derive_lexer_macro;

pub trait Tokenize {
    fn tokenize(input: &str) -> Option<(Self, &str)>
    where
        Self: Sized;
}

#[cfg(test)]
mod test {
    use crate::derive_lexer_macro::Tokenize;
    use crate::Tokenize;
    use regex;

    #[derive(Tokenize, Debug)]
    enum Token {
        #[lex(pattern = "^@\\w+")]
        Decorator,
        #[lex(pattern = "let\\s")]
        LetBinding,
    }

    #[test]
    fn can_derive_tokens() {
        let input = "@k let n";
        let mut curr_input = input;
        let mut tokens = Vec::new();

        while let Some((token, rest)) = Token::tokenize(curr_input) {
            tokens.push(token);
            curr_input = rest;
        }

        assert_eq!(tokens.len(), 2);

        dbg!(tokens);
    }
}
