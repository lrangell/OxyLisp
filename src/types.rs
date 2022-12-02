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
struct Lambda<'a> {
    args: HashMap<String, Primitive>,
    body: Vec<Form<'a>>,
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
pub enum Form<'a> {
    Symbol(String),
    Primitive(Primitive),
    // List(Vec<Form>),
    Expression(Expression<'a>),
    // Form(Box<Form>),
}

pub type Expression<'a> = (String, &'a [Form<'a>]);

#[derive(Clone)]
pub enum Objects {
    Primitive(Primitive),
    // Lambda(Lambda),
    BuiltinFn(BuiltinFn),
}
pub struct Env {
    pub vars: HashMap<String, Objects>,
}
