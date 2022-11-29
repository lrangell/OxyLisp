use crate::types::*;
use regex::Regex;

pub fn tokenize(expression: String) -> Vec<Tokens> {
    let is_symbol = Regex::new(r"[A-Za-z+*-]").unwrap();
    let is_integer = Regex::new(r"\d+").unwrap();

    return expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::OpeningParen,
                ")" => Tokens::ClosingParen,
                i if is_symbol.is_match(i) => Tokens::Symbol(i.to_string()),
                i if is_integer.is_match(i) => Tokens::Integer(i.parse::<i32>().unwrap()),
                _i => Tokens::Invalid,
            }
        })
        .collect::<Vec<_>>();
}

mod tests {
    use super::tokenize;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            vec![
                Tokens::OpeningParen,
                Tokens::Symbol("+".to_string()),
                Tokens::Integer(4),
                Tokens::Integer(2),
                Tokens::ClosingParen
            ],
            tokenize("(+ 4 2)".to_string())
        )
    }
}
