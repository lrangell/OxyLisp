#![allow(dead_code)]
#![allow(unused_variables)]
pub mod display;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use trees::Tree;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenBounds {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurlyBraces,
    RightCurlyBraces,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Bounds(TokenBounds),
    Literal(Literal),
    Symbol(String),
    Key(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Symbol(String),
    String(String),
    Integer(i32),
    Bool(bool),
    List(Vec<Literal>),
    Record(HashMap<String, Literal>),
}

pub type BuiltInFunction = fn(&[RuntimeObject]) -> Result<RuntimeObject>;

#[derive(Clone, PartialEq, Debug)]
pub enum Form {
    Root,
    Literal(Literal),
    CallExpression(String),
    Symbol(String),
    List,
    Record,
    Key(String),
}

pub type CallExpression = (String, Vec<Form>);

trait Eval {
    fn eval(&self) -> Form;
}

#[derive(Clone)]
pub struct Lambda {
    pub name: Option<String>,
    pub args: Vec<String>,
    pub body: Tree<Form>,
    pub env: Env,
    pub self_recursive: bool,
}

#[derive(Clone)]
pub enum RuntimeObject {
    NoOp,
    Primitive(Literal),
    List(Vec<RuntimeObject>),
    Record(HashMap<String, RuntimeObject>),
    Function(BuiltInFunction),
    RuntimeFunction(Lambda),
}

impl RuntimeObject {
    pub fn extract_primitive(self) -> Result<Literal> {
        match self {
            RuntimeObject::Primitive(p) => Ok(p),
            _ => Err(anyhow!("Error extracting primitive")),
        }
    }
    pub fn extract_number(self) -> Result<i32> {
        match self {
            RuntimeObject::Primitive(Literal::Integer(i)) => Ok(i),
            _ => Err(anyhow!("Error extracting primitive")),
        }
    }
    pub fn extract_bool(self) -> Result<bool> {
        match self {
            RuntimeObject::Primitive(Literal::Bool(b)) => Ok(b),
            _ => Err(anyhow!("Error extracting primitive")),
        }
    }
    pub fn extract_list(self) -> Result<Vec<RuntimeObject>> {
        match self {
            RuntimeObject::List(rt_vec) => Ok(rt_vec),
            _ => Err(anyhow!("Error extracting primitive")),
        }
    }
}

pub trait Primitive {
    fn extract(&self);
    fn extract_numbers(&self) -> Result<Vec<i32>>;
    fn extract_bools(&self) -> Result<Vec<bool>>;
    fn extract_lists(&self) -> Result<Vec<Vec<RuntimeObject>>>;
}

impl Primitive for &[RuntimeObject] {
    fn extract(&self) {
        todo!()
    }

    fn extract_numbers(&self) -> Result<Vec<i32>> {
        let a: Vec<i32> = self
            .iter()
            .map(|rto| match rto {
                RuntimeObject::Primitive(Literal::Integer(i)) => Ok(*i),
                _ => Err(anyhow!("Non integer object")),
            })
            .collect::<Result<Vec<i32>>>()?;
        Ok(a)
    }
    fn extract_bools(&self) -> Result<Vec<bool>> {
        let a: Vec<bool> = self
            .iter()
            .map(|rto| match rto {
                RuntimeObject::Primitive(Literal::Bool(b)) => Ok(*b),
                _ => Err(anyhow!("Non boolean object")),
            })
            .collect::<Result<Vec<bool>>>()?;
        Ok(a)
    }
    fn extract_lists(&self) -> Result<Vec<Vec<RuntimeObject>>> {
        self.iter()
            .map(|rto| match rto {
                //TODO: avoid clone
                RuntimeObject::List(l) => Ok(l.clone()),
                _ => Err(anyhow!("Non list object")),
            })
            .collect::<Result<Vec<Vec<RuntimeObject>>>>()
    }
}

#[derive(Clone)]
pub enum EnvType {
    RootEnv,
    LambdaEnv(Rc<Env>),
}

#[derive(Clone)]
pub struct Env {
    pub vars: RefCell<HashMap<String, RuntimeObject>>,
    pub parent: Option<Rc<Env>>,
}

pub type EnvPointer = Rc<Env>;

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

impl From<i32> for RuntimeObject {
    fn from(value: i32) -> Self {
        RuntimeObject::Primitive(Literal::Integer(value))
    }
}

impl From<bool> for RuntimeObject {
    fn from(value: bool) -> Self {
        RuntimeObject::Primitive(Literal::Bool(value))
    }
}

impl From<Form> for Literal {
    fn from(value: Form) -> Self {
        match value {
            Form::Literal(l) => l,
            Form::CallExpression(_) => todo!(),
            Form::Symbol(s) => todo!(),
            Form::List => todo!(),
            Form::Root => todo!(),
            Form::Record => todo!(),
            Form::Key(_) => todo!(),
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

impl From<Tokens> for Form {
    fn from(value: Tokens) -> Self {
        match value {
            Tokens::Bounds(_) => unreachable!(),
            Tokens::Literal(l) => Form::Literal(l),
            Tokens::Symbol(s) => Form::Symbol(s),
            Tokens::Key(k) => Form::Key(k),
        }
    }
}
