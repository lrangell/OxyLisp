#![allow(dead_code)]
#![allow(unused_variables)]
pub mod Errors;
pub mod display;
use anyhow::{anyhow, Result};
use std::{collections::HashMap, fmt};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenBounds {
    LeftParen,
    RightParen,
    LeftBracket, 
    RightBracket
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Bounds(TokenBounds),
    Literal(Literal),
    Symbol(String),
}

#[derive(Debug, Clone)]
struct Lambda {
    args: HashMap<String, Form>,
    body: Vec<Form>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i32),
    Bool(bool),
    List(Vec<Literal>),
}

pub type BuiltinFn = fn(&[Form], &mut Env) -> Result<Form>;

#[derive(Debug, Clone)]
pub enum Form {
    Literal(Literal),
    CallExpression(CallExpression),
    Symbol(String),
    List(Box<Vec<Form>>)
}

pub type CallExpression = (String, Vec<Form>);

trait Eval {
    fn eval(&self) -> Form;
}

#[derive(Clone)]
pub enum RuntimeObject {
    Primitive(Literal),
    Function(BuiltinFn),
}

pub struct Env {
    pub vars: HashMap<String, RuntimeObject>,
    // def: fn(Self, sym: Form, &RuntimeObject) -> Result<()>,
}

impl Env {
    pub fn def(&mut self, symbol: &Form, value: &RuntimeObject) -> Result<RuntimeObject> {
        let Form::Symbol(sym) = symbol else { 
            return Err(anyhow!("First argument of def must be a symbol"))
        };
        debug!("def sym: {} val: {}", sym, value);
        self.vars.insert(sym.clone(), value.clone());
        Ok(RuntimeObject::Primitive(Literal::Bool(true)))
    }
}

impl From<Literal> for Form {
    fn from(p: Literal) -> Self {
        Form::Literal(p)
    }
}
impl From<Literal> for RuntimeObject {
    fn from(p: Literal) -> Self {
        RuntimeObject::Primitive(p)
    }
}

impl From<Form> for Literal {
    fn from(value: Form) -> Self {
        match value {
            Form::Literal(l) => l,
            Form::CallExpression(_) => todo!(),
            Form::Symbol(s) => todo!(),
            Form::List(s) => todo!(),
        }
    }
}



impl From<i32> for Literal {
    fn from(p: i32) -> Self {
        Literal::Integer(p)
    }
}
impl From<i32> for Form {
    fn from(p: i32) -> Self {
        Form::Literal(Literal::Integer(p))
    }
}
