use std::collections::HashMap;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::*;
use log::{debug, info};

pub fn eval_form(form: &Form, env: &mut Box<Env>) -> Result<RuntimeObject> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone().into()),
        Form::Symbol(symbol) => {
            // info!("Env: {:?}, symbol: {symbol}", env.vars.keys());
            env.lookup(symbol)
                .ok_or(anyhow!("{symbol} is not defined "))
        }

        Form::CallExpression((s, forms)) if s == "def" => handle_def(forms, env),
        Form::CallExpression((s, forms)) if s == "defn" => handle_defn(forms, env),
        Form::CallExpression((s, forms)) if s == "if" => {
            match eval_form(&forms[0], env)?.extract_bool()? {
                true => eval_form(&forms[1], env),
                false => eval_form(&forms[2], env),
            }
        }
        Form::CallExpression((s, forms)) if s == "=" => Ok(Literal::Bool(
            eval_form(&forms[0], env)?.extract_primitive()?
                == eval_form(&forms[1], env)?.extract_primitive()?,
        )
        .into()),
        Form::List(forms) => {
            let literals: Result<Vec<RuntimeObject>> =
                forms.iter().map(|f| eval_form(f, env)).collect();
            Ok(RuntimeObject::List(literals?))
        }

        Form::CallExpression((to_call, forms)) => {
            let function = env
                .lookup(&to_call)
                .ok_or(anyhow!("Symbol {to_call} not defined"))?;

            match function {
                RuntimeObject::Function(f) => {
                    let args = eval_forms_vec(forms, env)?;
                    f(&args, env)
                }
                RuntimeObject::Primitive(p) => Ok(p.into()),
                RuntimeObject::RuntimeFunction(mut f) => {
                    let rt_forms: Vec<RuntimeObject> = eval_forms_vec(forms, env)?;
                    f.eval(rt_forms)
                }
                rto => Ok(rto),
            }
        }
    }
}

pub fn eval_forms_vec(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<Vec<RuntimeObject>> {
    forms.iter().map(|a| eval_form(a, env)).collect()
}

fn handle_def(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let (symbol, arg) = forms.split_first().unwrap();
    let value = eval_form(arg.first().unwrap(), env).unwrap();
    env.def(symbol, &value.into())?;
    Ok(Literal::Nil.into())
}
fn handle_defn(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let (symbol, rest) = forms.split_first().unwrap();
    let (args, forms) = rest.split_first().unwrap();
    let Form::List(arg_forms) = args else {return Err(anyhow!("arguments must be a list of symbols"))};
    let arguments: Vec<String> = arg_forms
        .iter()
        .map_while(|form| -> Option<String> {
            match form {
                Form::Symbol(s) => Some(s.to_string()),
                _ => None,
            }
        })
        .collect();
    env.defn(symbol, arguments, forms.to_vec())?;
    Ok(Literal::Nil.into())
}

pub fn eval(forms: Vec<Form>, env: &mut Box<Env>) -> Result<Vec<RuntimeObject>> {
    forms.iter().map(|f| eval_form(f, env)).collect()
}

pub fn eval_from_str(code: &str, env: &mut Box<Env>) -> Result<Vec<RuntimeObject>> {
    let tokens = tokenize(code);
    let forms = parse(&tokens)?;
    eval(forms, env)
}

impl Lambda {
    pub fn new(name: Option<String>, args: Vec<String>, body: Vec<Form>, parent: Box<Env>) -> Self {
        Lambda {
            name,
            args,
            body,
            env: Box::new(Env {
                vars: HashMap::new(),
                parent: EnvType::LambdaEnv(parent),
            }),
        }
    }
    pub fn bind_symbols(self: &mut Self, values: &Vec<RuntimeObject>) -> () {
        //TODO: check for arity
        self.args.iter().zip(values).for_each(|(k, v)| {
            self.env.vars.insert(k.to_string(), v.clone());
        });
        self.env.vars.insert(
            self.name.to_owned().unwrap(),
            RuntimeObject::RuntimeFunction(self.clone()),
        );
    }
    pub fn eval(self: &mut Self, args: Vec<RuntimeObject>) -> Result<RuntimeObject> {
        self.bind_symbols(&args);
        let result = eval_forms_vec(&self.body, &mut self.env)?
            .last()
            .unwrap_or(&Literal::Nil.into())
            .clone();
        Ok(result)
    }
}

// TODO: implement into for types
