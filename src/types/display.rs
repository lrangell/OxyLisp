use trees::Node;

use crate::prelude::*;

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounds(b) => write!(f, "{}", b),
            Self::Literal(p) => write!(f, "{}", p),
            Self::Symbol(p) => write!(f, "{}", p),
        }
    }
}
impl fmt::Display for TokenBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
        }
    }
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Form::Literal(p) => write!(f, "{}", p),
            Form::Symbol(s) => write!(f, "{}", s),
            Form::CallExpression(s) => {
                write!(f, "{}", s)
            }
            Form::List => {
                write!(f, "[]",)
            }
            Form::Root => write!(f, "",),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Integer(s) => write!(f, "{s}"),
            Literal::Bool(s) => write!(f, "{s}"),
            Literal::Nil => write!(f, "Nil"),
            Literal::List(s) => {
                let literal_strings: Vec<String> = s.iter().map(|p| p.to_string()).collect();
                write!(f, "[{}]", literal_strings.join(" "))
            }
            Literal::Symbol(s) => write!(f, "{s}"),
        }
    }
}
impl fmt::Display for RuntimeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeObject::Primitive(p) => write!(f, "{}", p),
            RuntimeObject::Function(p) => write!(f, "#function#",),
            RuntimeObject::RuntimeFunction(p) => write!(f, "#function#",),
            RuntimeObject::List(s) => {
                let literal_strings: Vec<String> = s.iter().map(|p| p.to_string()).collect();
                write!(f, "[{}]", literal_strings.join(" "))
            }
            RuntimeObject::NoOp => write!(f, "",),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self
            .vars
            .borrow()
            .clone()
            .into_iter()
            .map(|(sym, val)| format!("{} {}", sym, val))
            .collect();
        write!(f, "{}", items.join("\n"))
    }
}

pub trait PrintAST {
    fn _print_ast(&self, acc: &mut String) -> Result<String>;
    fn print_ast(&self) -> Result<String>;
}

use core::fmt;
use std::fmt::Write;
impl PrintAST for Node<Form> {
    #[allow(unused_must_use)]
    fn _print_ast(&self, acc: &mut String) -> Result<String> {
        match self.data() {
            Form::Literal(l) => write!(acc, " {l}"),
            Form::CallExpression(funcion_name) => {
                write!(acc, " ({funcion_name}")?;
                self.iter().for_each(|n| {
                    n._print_ast(acc);
                });
                write!(acc, ")")
            }
            Form::Symbol(s) => write!(acc, " {s}"),
            Form::List => {
                write!(acc, " [")?;
                self.iter().for_each(|n| {
                    n._print_ast(acc);
                });
                write!(acc, "] ")
            }
            Form::Root => {
                self.iter().for_each(|n| {
                    n._print_ast(acc);
                });
                write!(acc, "")
            }
        }?;
        Ok(acc
            .replace("[ ", "[")
            .replace("] ", "]")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" "))
    }

    fn print_ast(&self) -> Result<String> {
        self._print_ast(&mut "".to_string())
    }
}
