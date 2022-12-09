use crate::types::codeChunk::CodeChunk;
use crate::types::*;
use anyhow::Result;
use anyhow::*;

pub fn eval(code: CodeChunk, env: &Env) -> Result<Vec<Literal>> {
    debug!("Code: {code}");
    let a: Result<Vec<Literal>> = code.into_iter().map(|form| eval_form(&form, env)).collect();
    a
}

pub fn eval_form(form: &Form, env: &Env) -> Result<Literal> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone()),
        Form::Symbol(s) => {
            let obj = env.vars.get(s).expect("aaa");
            match obj {
                Objects::Literal(p) => Ok(p.clone()),
                _ => Err(anyhow!("Symbol not defined")),
            }
        }
        Form::Expression((to_call, forms)) => {
            let function = env.vars.get(to_call).expect("not defined");
            match function {
                Objects::BuiltinFn(f) => {
                    f(forms.iter().map(|a| eval_form(a, env).unwrap()).collect())
                }
                Objects::Literal(p) => Ok(p.clone()),
            }
        }
    }
}
// TODO: implement into for types
