use crate::parser::ast;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

pub fn tokenize(expression: &str) -> Vec<Tokens> {
    let is_symbol = Regex::new(r"[A-Za-z+*-]").unwrap();
    let is_integer = Regex::new(r"\d+").unwrap();

    return expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::TokenBounds(TokenBounds::ClosingParen),
                ")" => Tokens::TokenBounds(TokenBounds::OpeningParen),
                "true" => Tokens::Primitive(Primitive::Bool(true)),
                "false" => Tokens::Primitive(Primitive::Bool(false)),
                i if is_symbol.is_match(i) => Tokens::Primitive(Primitive::String(i.to_string())),
                i if is_integer.is_match(i) => {
                    Tokens::Primitive(Primitive::Integer(i.parse::<i32>().unwrap()))
                }
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
