use crate::types::codeChunk::CodeChunk;
use crate::types::*;
use anyhow::Result;
use anyhow::*;
use std::collections::HashMap;

pub fn eval(code: CodeChunk, env: &Env) -> Result<Vec<Primitive>> {
    unimplemented!();
    let a: Result<Vec<Primitive>> = code.into_iter().map(|form| eval_form(&form, env)).collect();
}

pub fn eval_form(form: &Form, env: &Env) -> Result<Primitive> {
    match form {
        Form::Primitive(p) => Ok(p.clone()),
        Form::Symbol(s) => {
            let obj = env.vars.get(s).expect("aaa");
            match obj {
                Objects::Primitive(p) => Ok(p.clone()),
                _ => Err(anyhow!("Symbol not defined")),
            }
        }
        Form::Expression((toCall, forms)) => {
            let function = env.vars.get(toCall).expect("not defined");
            match function {
                Objects::BuiltinFn(f) => {
                    f(forms.iter().map(|a| eval_form(a, env).unwrap()).collect())
                }
                Objects::Primitive(p) => Ok(p.clone()),
            }
        }
    }
}
// TODO: implement into for types
