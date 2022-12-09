use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::Result;
use anyhow::*;

pub fn eval_form(form: &Form, env: &Env) -> Result<Literal> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone()),
        Form::Symbol(s) => {
            let obj = env.vars.get(s).expect("aaa");
            match obj {
                RuntimeObject::Primitive(p) => Ok(p.clone()),
                _ => unimplemented!(),
            }
        }
        Form::CallExpression((to_call, forms)) => {
            let function = env.vars.get(to_call).expect("not defined");
            match function {
                RuntimeObject::Function(f) => {
                    f(forms.iter().map(|a| eval_form(a, env).unwrap()).collect())
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
