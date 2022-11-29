use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tokens {
    Invalid,
    OpeningParen,
    ClosingParen,
    Integer(i32),
    Symbol(String),
}

#[derive(Debug, Clone)]
struct Lambda {
    args: HashMap<String, Primitive>,
    body: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    String(String),
    Symbol(String),
    Integer(i32),
    Bool(bool),
}

pub type BuiltinFn = fn(&[Primitive]) -> Result<Primitive>;

#[derive(Debug, Clone)]
pub enum Expression {
    Primitive(Primitive),
    List(Vec<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub struct Env {
    vars: HashMap<String, Primitive>,
}
