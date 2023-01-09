use crate::prelude::*;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

lazy_static! {
    static ref IS_SYMBOL: Regex = Regex::new(r"[A-Za-z+*-=]").unwrap();
    static ref IS_STRING: Regex = Regex::new("\".*\"").unwrap();
    static ref IS_INTEGER: Regex = Regex::new("\".*\"").unwrap();
}

pub fn tokenize(expression: &str) -> Vec<Tokens> {
    let is_symbol = Regex::new(r"[A-Za-z+*-=]").unwrap();
    let is_integer = Regex::new(r"\d+").unwrap();
    let is_string = Regex::new("\".*\"").unwrap();

    let tokens = expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::Bounds(TokenBounds::LeftParen),
                ")" => Tokens::Bounds(TokenBounds::RightParen),
                "[" => Tokens::Bounds(TokenBounds::LeftBracket),
                "]" => Tokens::Bounds(TokenBounds::RightBracket),
                "true" => Tokens::Literal(Literal::Bool(true)),
                "false" => Tokens::Literal(Literal::Bool(false)),
                i if is_string.is_match(i) => Tokens::Literal(Literal::String(i.to_string())),
                i if is_integer.is_match(i) => {
                    Tokens::Literal(Literal::Integer(i.parse::<i32>().unwrap()))
                }
                i if is_symbol.is_match(i) => Tokens::Symbol(i.to_string()),
                _ => panic!("invalid token"),
            }
        })
        .collect::<Vec<_>>();
    debug!("Tokenizing code:\n {}", expression);
    debug!("Tokens: {:?}", tokens);
    tokens
}
