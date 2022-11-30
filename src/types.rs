#![allow(dead_code)]
#![allow(unused_variables)]
use anyhow::Result;
use std::{collections::HashMap, fmt};

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

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::String(b) => write!(f, "{}", b),
            Primitive::Symbol(b) => write!(f, "{}", b),
            Primitive::Integer(b) => write!(f, "{}", b),
            Primitive::Bool(b) => write!(f, "{}", b),
        }
        // write!(f, "{}", *self)
    }
}

pub type BuiltinFn = fn(&[Primitive]) -> Result<Primitive>;

#[derive(Debug, Clone)]
pub enum Expression {
    Primitive(Primitive),
    List(Vec<Expression>),
    Expression(Box<Expression>),
}

#[derive(Clone)]
pub enum Objects {
    Primitive(Primitive),
    Lambda(Lambda),
    BuiltinFn(BuiltinFn),
}
pub struct Env {
    pub vars: HashMap<String, Objects>,
}
