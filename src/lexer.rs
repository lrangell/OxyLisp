use crate::types::*;
use regex::Regex;

pub fn tokenize(expression: &str) -> Vec<Tokens> {
    let is_symbol = Regex::new(r"[A-Za-z+*-]").unwrap();
    let is_integer = Regex::new(r"\d+").unwrap();
    let is_string = Regex::new("\".*\"").unwrap();

    let tokens = expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::Bounds(TokenBounds::LeftParen),
                ")" => Tokens::Bounds(TokenBounds::RightParen),
                "true" => Tokens::Literal(Literal::Bool(true)),
                "false" => Tokens::Literal(Literal::Bool(false)),
                i if is_symbol.is_match(i) => Tokens::Symbol(i.to_string()),
                i if is_string.is_match(i) => Tokens::Literal(Literal::String(i.to_string())),
                i if is_integer.is_match(i) => {
                    Tokens::Literal(Literal::Integer(i.parse::<i32>().unwrap()))
                }
                _ => panic!("invalid token"),
            }
        })
        .collect::<Vec<_>>();
    debug!("Tokenizing code:\n {}", expression);
    debug!("Tokens: {:?}", tokens);
    tokens
}
