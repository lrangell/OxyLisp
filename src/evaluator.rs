use std::borrow::Borrow;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::*;

pub fn eval_form(form: &Form, env: &Env) -> Result<Literal> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone()),
        Form::Symbol(s) => {
            let obj = env.vars.get(s).expect("aaa");
            match obj {
                RuntimeObject::Primitive(p) => Ok(p.clone()),
                _ => Err(anyhow!("")),
            }
        }
        Form::CallExpression((to_call, forms)) => {
            let function = env.vars.get(to_call).expect("not defined");
            match function {
                RuntimeObject::Function(f) => {
                    let args: Result<Vec<Form>> = forms
                        .iter()
                        .map(|a| Ok(eval_form(a, env)?.into()))
                        .collect();
                    let returned_value = f(args?.as_slice())?;
                    eval_form(&returned_value.borrow(), env)
                }
                RuntimeObject::Primitive(p) => Ok(p.clone()),
            }
        }
    }
}
pub fn eval(forms: Vec<Form>, env: &Env) -> Result<Vec<Literal>> {
    forms.iter().map(|f| eval_form(f, env)).collect()
}

pub fn eval_from_str(code: &str, env: &Env) -> Result<Vec<Literal>> {
    let tokens = tokenize(code);
    let forms = parse(&tokens)?;
    eval(forms, env)
}

// TODO: implement into for types
