use std::{borrow::Borrow, collections::HashMap};

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::types::*;
use anyhow::*;
use log::{debug, info};

pub fn eval_form(form: &Form, env: &mut Box<Env>) -> Result<RuntimeObject> {
    debug!("Form: {form}");
    match form {
        Form::Literal(p) => Ok(p.to_owned().into()),
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
        Form::CallExpression((s, forms)) if s == "get" => get(forms, env),
        Form::CallExpression((s, forms)) if s == "push" => push(forms, env),
        Form::CallExpression((s, forms)) if s == "map" => map(forms, env),

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

fn map(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<RuntimeObject, Error> {
    let rtos = eval_forms_vec(forms, env)?;
    let list = rtos[1].clone().extract_list()?;
    if let RuntimeObject::RuntimeFunction(mut f) = rtos[0].to_owned() {
        let mapped: Vec<RuntimeObject> = list
            .iter()
            .map(|el| f.eval(vec![el.clone()]))
            .collect::<Result<Vec<RuntimeObject>>>()?;
        return Ok(RuntimeObject::List(mapped));
    }
    Err(anyhow!("map error"))
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

fn get(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let rtos = eval_forms_vec(forms, env)?;
    let index = rtos[0].clone().extract_primitive()?;
    let list = rtos[1].clone().extract_list()?;
    match index {
        Literal::Integer(i) if i < list.len() as i32 => Ok(list[i as usize].clone()),
        _ => Err(anyhow!("error")),
    }
}
fn push(forms: &Vec<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let rtos = eval_forms_vec(forms, env)?;
    let mut list = rtos[1].clone().extract_list()?;
    list.push(rtos[0].clone());
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
    pub fn bind_symbols(self: &mut Self, values: Vec<RuntimeObject>) -> () {
        //TODO: check for arity
        self.env.vars.extend(self.args.iter().cloned().zip(values));
        self.env.vars.insert(
            self.name.to_owned().unwrap(),
            RuntimeObject::RuntimeFunction(self.to_owned()),
        );
    }
    pub fn eval(self: &mut Self, args: Vec<RuntimeObject>) -> Result<RuntimeObject> {
        let remaing_args = self.args.len() - args.len();
        if remaing_args > 1 {
            let mut partial_fn = self.clone();
            partial_fn.bind_symbols(args);
            partial_fn.args = self
                .args
                .clone()
                .into_iter()
                .skip(self.args.len() - remaing_args)
                .collect();
            return Ok(RuntimeObject::RuntimeFunction(partial_fn));
        };
        self.bind_symbols(args);
        let result = eval_forms_vec(&self.body.clone(), &mut self.env)?
            .last()
            .unwrap_or(&Literal::Nil.into())
            .to_owned();
        Ok(result)
    }
    pub fn tail_eval(self: &mut Self, args: Vec<RuntimeObject>) -> Result<RuntimeObject> {
        todo!()
    }
}

// TODO: implement into for types
