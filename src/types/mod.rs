#![allow(dead_code)]
#![allow(unused_variables)]
pub mod display;
use anyhow::{anyhow, Result};
use log::debug;
use std::{collections::HashMap, fmt};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenBounds {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Bounds(TokenBounds),
    Literal(Literal),
    Symbol(String),
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
    List(Box<Vec<Form>>),
}

pub type CallExpression = (String, Vec<Form>);

trait Eval {
    fn eval(&self) -> Form;
}

#[derive(Clone)]
pub struct Lambda {
    pub name: Option<String>,
    pub args: Vec<String>,
    pub body: Vec<Form>,
    pub env: Env,
}

#[derive(Clone)]
pub enum RuntimeObject {
    Primitive(Literal),
    Function(BuiltinFn),
    RuntimeFunction(Lambda),
}

#[derive(Clone)]
pub enum EnvType {
    RootEnv,
    LambdaEnv(Box<Env>),
}

#[derive(Clone)]
pub struct Env {
    pub vars: HashMap<String, RuntimeObject>,
    pub parent: EnvType,
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
    pub fn defn(
        self: &mut Box<Self>,
        symbol: &Form,
        arguments: Vec<String>,
        forms: Vec<Form>,
    ) -> Result<RuntimeObject> {
        let Form::Symbol(sym) = symbol else {
            return Err(anyhow!("First argument of defn must be a symbol"))
        };
        let function = Lambda::new(
            Some(symbol.to_string()),
            arguments.iter().map(|a| a.to_string()).collect(),
            forms,
            &self,
        );
        self.vars.insert(
            sym.to_string(),
            RuntimeObject::RuntimeFunction(function).clone(),
        );
        Ok(RuntimeObject::Primitive(Literal::Bool(true)))
    }

    pub fn lookup(&self, symbol: &str) -> Option<RuntimeObject> {
        debug!("Env: {self}");
        if let Some(value) = self.vars.get(symbol) {
            return Some(value.clone());
        }
        match &self.parent {
            EnvType::RootEnv => None,
            EnvType::LambdaEnv(parent) => parent.lookup(symbol),
        }
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
