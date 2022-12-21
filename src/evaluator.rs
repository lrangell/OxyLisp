use std::borrow::Borrow;
use std::collections::HashMap;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::*;
use log::{debug, info};

pub fn eval_form(form: &Form, env: &mut Box<Env>) -> Result<Literal> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.clone()),
        Form::Symbol(symbol) => {
            info!("Env: {:?}, symbol: {symbol}", env.vars.keys());
            let obj = env
                .lookup(symbol)
                .ok_or(anyhow!("{symbol} is not defined "))?;
            match obj {
                RuntimeObject::Primitive(p) => Ok(p.clone()),
                _ => Err(anyhow!("Runtime object can't be evaluated")),
            }
        }

        Form::CallExpression((s, forms)) if s == "def" => handle_def(forms, env),
        Form::CallExpression((s, forms)) if s == "defn" => handle_defn(forms, env),
        Form::CallExpression((s, forms)) if s == "if" => match eval_form(&forms[0], env)? {
            Literal::String(_) => todo!(),
            Literal::Integer(_) => todo!(),
            Literal::Bool(true) => eval_form(&forms[1], env),
            Literal::Bool(false) => eval_form(&forms[2], env),
            Literal::List(_) => todo!(),
        },
        Form::CallExpression((s, forms)) if s == "=" => Ok(Literal::Bool(
            eval_form(&forms[0], env)? == eval_form(&forms[1], env)?,
        )),
        Form::List(forms) => {
            let literals: Result<Vec<Literal>> = forms.iter().map(|f| eval_form(f, env)).collect();
            Ok(Literal::List(literals?))
        }

        Form::CallExpression((to_call, forms)) => {
            let function = env
                .lookup(&to_call)
                .ok_or(anyhow!("Symbol {to_call} not defined"))?;

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
                RuntimeObject::RuntimeFunction(mut f) => {
                    let rt_forms: Vec<RuntimeObject> = eval_forms_vec(forms, env)?
                        .iter()
                        .map(|i| Into::<RuntimeObject>::into(i.clone()))
                        .collect();
                    f.eval(rt_forms)
                }
            }
        }
    }
}

pub fn eval_forms_vec(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<Vec<Literal>> {
    let rt_objects: Result<Vec<Literal>> = forms
        .iter()
        .map(|a| Ok(eval_form(a, env)?.into()))
        .collect();
    rt_objects
}

fn handle_def(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<Literal> {
    debug!("Evaluating def expresssion: forms: {:?}", forms);
    let (symbol, arg) = forms.split_first().unwrap();
    let value = eval_form(arg.first().unwrap(), env).unwrap();
    env.def(symbol, &value.into())?;
    Ok(Literal::Bool(true))
}
fn handle_defn(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<Literal> {
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
    Ok(Literal::Bool(true))
}

pub fn eval(forms: Vec<Form>, env: &mut Box<Env>) -> Result<Vec<Literal>> {
    forms.iter().map(|f| eval_form(f, env)).collect()
}

pub fn eval_from_str(code: &str, env: &mut Box<Env>) -> Result<Vec<Literal>> {
    let tokens = tokenize(code);
    let forms = parse(&tokens)?;
    eval(forms, env)
}

impl Lambda {
    pub fn new(
        name: Option<String>,
        args: Vec<String>,
        body: Vec<Form>,
        parent: &Box<Env>,
    ) -> Self {
        Lambda {
            name,
            args,
            body,
            env: Env {
                vars: HashMap::new(),
                parent: EnvType::LambdaEnv(parent.clone()),
            },
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
    pub fn eval(self: &mut Self, args: Vec<RuntimeObject>) -> Result<Literal> {
        self.bind_symbols(&args);
        let mut e = Box::new(self.env.clone());
        let a = eval_forms_vec(&self.body, &mut e)?;
        a.first().ok_or(anyhow!("aaa bbb")).cloned()
    }
}

// TODO: implement into for types
