mod built_in_functions;
mod lambda;

use crate::{parser::parse_string, prelude::*};
use anyhow::*;
// use log::{debug, info};
use trees::Node;

pub fn eval(node: &mut Node<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    match node.data().to_owned() {
        Form::Root => Ok(eval_children(node, env)?.iter().last().unwrap().to_owned()),
        Form::CallExpression(symbol) => match symbol.as_str() {
            "def" => def(node, env),
            "defn" => defn(node, env),
            "fn" => build_lambda(node, env, None),
            "if" => eval_if(node, env),
            _ => eval_call_expr(symbol, node, env),
        },
        Form::List => eval_list(node, env),
        Form::Symbol(symbol) => env
            .lookup(&symbol)
            .ok_or(anyhow!("{symbol} is not defined ")),
        Form::Literal(l) => Ok(l.to_owned().into()),
    }
}
// TODO: handle def and defn
//
//
fn def(node: &mut Node<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let var_name_node = node.pop_front().unwrap();
    let symbol = var_name_node.data();
    let value = eval(&mut node.front_mut().unwrap(), env)?;
    env.vars.insert(symbol.to_string(), value);
    Ok(RuntimeObject::NoOp)
}
fn defn(node: &mut Node<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let function_name_node = node.pop_front().unwrap();

    let Form::Symbol(symbol) = function_name_node.data() else {
        return Err(anyhow!("First argument of defn must be a symbol"))
    };

    let f = build_lambda(node, env, Some(symbol.to_string()))?;

    env.vars.insert(symbol.to_string(), f);
    Ok(RuntimeObject::NoOp)
}

fn build_lambda(
    node: &mut Node<Form>,
    env: &mut Box<Env>,
    name: Option<String>,
) -> Result<RuntimeObject> {
    let args = node
        .pop_front()
        .unwrap()
        .iter()
        .map(|n| match n.data() {
            Form::Symbol(sym) => Ok(sym.to_owned()),
            _ => Err(anyhow!(
                "Arguments of a function declaration must be symbols"
            )),
        })
        .collect::<Result<Vec<String>>>()?;
    let body = node.detach();
    let f = Lambda::new(name, args, body, env.to_owned());
    Ok(RuntimeObject::RuntimeFunction(f))
}
fn eval_if(node: &mut Node<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    //TODO: check for number of arguments
    let mut conditional_node = node.pop_front().unwrap();
    let res = eval(&mut conditional_node.root_mut(), env)?;
    match res {
        RuntimeObject::Primitive(Literal::Bool(true)) => eval(&mut node.front_mut().unwrap(), env),
        RuntimeObject::Primitive(Literal::Bool(false)) => eval(&mut node.back_mut().unwrap(), env),
        _ => Err(anyhow!("First form of a if must eval to a boolean")),
    }
}

fn eval_call_expr(
    name: String,
    form: &mut Node<Form>,
    env: &mut Box<Env>,
) -> Result<RuntimeObject> {
    let args = eval_children(form, env)?;

    if let Some(f) = built_in_functions::FUNCTIONS.get(name.as_str()) {
        return f(&args, env);
    }

    if let Some(RuntimeObject::RuntimeFunction(mut f)) = env.lookup(&name) {
        return f.eval(&args);
    }

    return Err(anyhow!("{name} is not defined"));
}
fn eval_list(form: &mut Node<Form>, env: &mut Box<Env>) -> Result<RuntimeObject> {
    eval_children(form, env).map(RuntimeObject::List)
}

fn eval_children(form: &mut Node<Form>, env: &mut Box<Env>) -> Result<Vec<RuntimeObject>> {
    form.iter_mut()
        .map(|mut node| eval(&mut node, env))
        .collect::<Result<Vec<RuntimeObject>>>()
}
pub fn eval_str(code: &str, env: &mut Box<Env>) -> Result<RuntimeObject> {
    let mut ast = parse_string(code)?;
    let mut root = ast.root_mut();
    eval(&mut root, env)
}
