use std::borrow::Borrow;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::*;

pub fn eval_form(form: &Form, env: &mut Env) -> Result<Literal> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone()),
        Form::Symbol(s) => {
            debug!("Env: {:?}, s: {s}", env.vars.keys());
            let obj = env.vars.get(s).expect("aaa");
            match obj {
                RuntimeObject::Primitive(p) => Ok(p.clone()),
                _ => Err(anyhow!("")),
            }
        }

        Form::CallExpression((defstring, forms)) if defstring == "def" => {
            debug!("Evaluating def expresssion: form: {:?}", forms);
            if let RuntimeObject::Function(def_fn) = env.vars.get("def").unwrap().clone() {
                let (sym, arg) = forms.split_first().unwrap();
                let arg_value = eval_form(arg.first().unwrap(), env)?;
                let result = def_fn([sym.clone(), arg_value.into()].as_slice(), env)?;
                Ok(result.into())
                // todo!()
            } else {
                Err(anyhow!("aa"))
            }
        }

        Form::CallExpression((to_call, forms)) => {
            let function = env.vars.get(to_call).cloned().expect("");
            match function {
                RuntimeObject::Function(f) => {
                    let args: Result<Vec<Form>> = forms
                        .iter()
                        .map(|a| Ok(eval_form(a, env)?.into()))
                        .collect();
                    let returned_value = f(args?.as_slice(), env)?;
                    eval_form(&returned_value.borrow(), env)
                }
                RuntimeObject::Primitive(p) => Ok(p.clone()),
            }
        }
    }
}

pub fn eval(forms: Vec<Form>, env: &mut Env) -> Result<Vec<Literal>> {
    forms.iter().map(|f| eval_form(f, env)).collect()
}

pub fn eval_from_str(code: &str, env: &mut Env) -> Result<Vec<Literal>> {
    let tokens = tokenize(code);
    let forms = parse(&tokens)?;
    eval(forms, env)
}

// TODO: implement into for types
