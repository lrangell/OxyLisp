#![allow(dead_code)]
#![allow(unused_variables)]
pub mod Errors;
pub mod codeChunk;
pub mod display;
use anyhow::Result;
use std::{collections::HashMap, fmt};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenBounds {
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
pub enum Tokens {
    Bounds(TokenBounds),
    Literal(Literal),
    Symbol(String),
}

#[derive(Debug, Clone)]
struct Lambda {
    args: HashMap<String, Literal>,
    body: Vec<Form>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(i32),
    Bool(bool),
}

pub type BuiltinFn = fn(Vec<Literal>) -> Result<Literal>;

#[derive(Debug, Clone)]
pub enum Form {
    Literal(Literal),
    CallExpression(CallExpression),
    Symbol(String),
}

pub type CallExpression = (String, Vec<Form>);

trait Eval {
    fn eval(&self) -> Form;
}

pub enum RuntimeObject {
    Primitive(Literal),
    Function(BuiltinFn),
}
pub struct Env {
    pub vars: HashMap<String, RuntimeObject>,
}

impl From<Literal> for Form {
    fn from(p: Literal) -> Self {
        Form::Literal(p)
    }
}
