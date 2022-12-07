use crate::parser::ast;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

pub fn tokenize(expression: &str) -> Vec<Tokens> {
    let is_symbol = Regex::new(r"[A-Za-z+*-]").unwrap();
    let is_integer = Regex::new(r"\d+").unwrap();

    let tokens = expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::TokenBounds(TokenBounds::OpeningParen),
                ")" => Tokens::TokenBounds(TokenBounds::ClosingParen),
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
    debug!("Tokenizing code:\n {}", expression);
    debug!("Tokens: {:?}", tokens);
    tokens
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
