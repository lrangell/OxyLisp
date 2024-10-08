use crate::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref IS_SYMBOL: Regex = Regex::new(r"[A-Za-z+*-=]").unwrap();
    static ref IS_KEY: Regex = Regex::new(r":[A-Za-z+*-=]").unwrap();
    static ref IS_STRING: Regex = Regex::new("\".*\"").unwrap();
    static ref IS_INTEGER: Regex = Regex::new(r"\d+").unwrap();
}

pub fn tokenize(expression: &str) -> Vec<Tokens> {
    expression
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace('[', " [ ")
        .replace(']', " ] ")
        .replace('{', " { ")
        .replace('}', " } ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::Bounds(TokenBounds::LeftParen),
                ")" => Tokens::Bounds(TokenBounds::RightParen),
                "[" => Tokens::Bounds(TokenBounds::LeftBracket),
                "]" => Tokens::Bounds(TokenBounds::RightBracket),
                "{" => Tokens::Bounds(TokenBounds::LeftCurlyBraces),
                "}" => Tokens::Bounds(TokenBounds::RightCurlyBraces),
                "true" => Tokens::Literal(Literal::Bool(true)),
                "false" => Tokens::Literal(Literal::Bool(false)),
                i if IS_KEY.is_match(i) => Tokens::Key(i[1..].to_string()),
                i if IS_STRING.is_match(i) => Tokens::Literal(Literal::String(i.to_string())),
                i if IS_INTEGER.is_match(i) => {
                    Tokens::Literal(Literal::Integer(i.parse::<i32>().unwrap()))
                }
                i if IS_SYMBOL.is_match(i) => Tokens::Symbol(i.to_string()),
                _ => panic!("invalid token"),
            }
        })
        .collect::<Vec<_>>()
}
