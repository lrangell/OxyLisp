#![allow(dead_code)]
#![allow(unused_variables)]
use anyhow::Result;
use std::{collections::HashMap, fmt};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TokenBounds {
    OpeningParen,
    ClosingParen,
}
#[derive(Debug, Clone)]
pub enum Tokens {
    Invalid,
    TokenBounds(TokenBounds),
    Primitive(Primitive),
}

#[derive(Debug, Clone)]
struct Lambda {
    args: HashMap<String, Primitive>,
    body: Vec<Form>,
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

pub type BuiltinFn = fn(Vec<Primitive>) -> Result<Primitive>;

#[derive(Debug, Clone)]
pub enum Form {
    Symbol(String),
    Primitive(Primitive),
    // List(Vec<Form>),
    Expression(Expression),
    // Form(Box<Form>),
}

pub type Expression = (String, Vec<Form>);

#[derive(Clone)]
pub enum Objects {
    Primitive(Primitive),
    // Lambda(Lambda),
    BuiltinFn(BuiltinFn),
}
pub struct Env {
    pub vars: HashMap<String, Objects>,
}

pub struct ParsingError;

impl From<Primitive> for Form {
    fn from(p: Primitive) -> Self {
        Form::Primitive(p)
    }
}
