#![allow(dead_code)]
#![allow(unused_variables)]
pub mod Errors;
pub mod codeChunk;
use anyhow::Result;
use std::{collections::HashMap, fmt, ptr::write};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenBounds {
    OpeningParen,
    ClosingParen,
}
impl fmt::Display for TokenBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpeningParen => write!(f, "("),
            Self::ClosingParen => write!(f, ")"),
        }
        // write!(f, "{}", *self)
    }
}
#[derive(Debug, Clone)]
pub enum Tokens {
    Invalid,
    TokenBounds(TokenBounds),
    Primitive(Primitive),
}
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid Token"),
            Self::TokenBounds(TokenBounds::OpeningParen) => write!(f, "("),
            Self::TokenBounds(TokenBounds::ClosingParen) => write!(f, ")"),
            Self::Primitive(p) => write!(f, "{}", p),
        }
        // write!(f, "{}", *self)
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "{s}"),
            Primitive::Integer(s) => write!(f, "{s}"),
            Primitive::Symbol(s) => write!(f, "{s}"),
            Primitive::Bool(s) => write!(f, "{s}"),
        }
        // write!(f, "{}", *self)
    }
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

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Form::Primitive(p) => write!(f, "{}", p),
            Form::Symbol(s) => write!(f, "{}", s),
            Form::Expression((to_call, forms)) => {
                let forms_string: Vec<String> = forms.iter().map(|f| f.to_string()).collect();
                write!(f, "({} {})", to_call, forms_string.join(" "))
            }
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
// impl From<Form> for Primitive {
//     fn from(f: Form) -> Self {
//         match f {
//             Form::Expression(e) => codeChunk::CodeChunk { tokens: vec![] },
//         }
//     }
// }
