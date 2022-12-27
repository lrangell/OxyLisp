mod built_in_functions;
mod lambda;

use std::{
    borrow::{Borrow, BorrowMut},
    ops::Deref,
    rc::Rc,
};

use crate::{
    parser::parse_string,
    prelude::{display::PrintAST, *},
};
use anyhow::*;
use once_cell::sync::Lazy;
// use log::debug;
// use log::{debug, info};
use trees::{Forest, Node, Tree};

pub fn eval(node: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    match node.data().to_owned() {
        Form::Root => unreachable!(),
        Form::CallExpression(symbol) => match symbol.as_str() {
            "if" => eval_if(node, env),
            "def" => def(node, env),
            "defn" => defn(node, env),
            "fn" => build_lambda(node, env, None),
            _ => eval_call_expr(symbol, node, env),
        },
        Form::List => eval_list(node, env),
        Form::Symbol(symbol) if symbol == "env" => Ok(RuntimeObject::NoOp),
        Form::Symbol(symbol) => env
            .lookup(&symbol)
            .ok_or(anyhow!("{symbol} is not defined ")),
        Form::Literal(l) => Ok(l.to_owned().into()),
    }
}

fn def(node: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    let mut body = node.deep_clone();
    let symbol_node = body.pop_front().unwrap();
    let value = eval(body.front().unwrap(), env.clone())?;

    env.vars
        .borrow_mut()
        .insert(symbol_node.data().to_string(), value);

    Ok(RuntimeObject::NoOp)
}
fn defn(node: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    let mut body = node.deep_clone();
    let function_name_node = body.pop_front().unwrap();

    let Form::Symbol(symbol) = function_name_node.data() else {
        return Err(anyhow!("First argument of defn must be a symbol"))
    };

    let lambda_args = body.pop_front().unwrap();
    let lambda_body = body.pop_front().unwrap();

    let mut lambda_node = Tree::new(Form::CallExpression("".to_string()));
    lambda_node.push_front(lambda_body);
    lambda_node.push_front(lambda_args);

    let f = build_lambda(&lambda_node, env.clone(), Some(symbol.to_string()))?;

    env.vars.borrow_mut().insert(symbol.to_string(), f);

    Ok(RuntimeObject::NoOp)
}

fn build_lambda(node: &Node<Form>, env: EnvPointer, name: Option<String>) -> Result<RuntimeObject> {
    let mut body = node.deep_clone_forest();
    let args_node = body.pop_front().unwrap();
    let args = args_node
        .root()
        .iter()
        .map(|n| match n.data() {
            Form::Symbol(sym) => Ok(sym.to_owned()),
            _ => Err(anyhow!(
                "Arguments of a function declaration must be symbols"
            )),
        })
        .collect::<Result<Vec<String>>>()?;

    // let lambda_body = body

    let f = Lambda::new(name, args, body, env);
    Ok(RuntimeObject::RuntimeFunction(f))
}
fn eval_if(node: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    //TODO: check for number of arguments
    if node.degree() != 3 {
        return Err(anyhow!("If form must have 3 arguments"));
    }

    let mut args = node.deep_clone();
    let cond = args.pop_front().unwrap();
    let res = eval(cond.root(), env.clone())?;

    let choosen_arm = match res {
        RuntimeObject::Primitive(Literal::Bool(true)) => args.front().unwrap(),
        RuntimeObject::Primitive(Literal::Bool(false)) => args.back().unwrap(),
        _ => todo!(),
    };

    let res = eval(choosen_arm, env.clone());
    res
}

fn eval_call_expr(name: String, form: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    let args = eval_children(form, env.clone())?;

    if let Some(f) = built_in_functions::FUNCTIONS.get(name.as_str()) {
        return f(&args);
    }

    if let Some(RuntimeObject::RuntimeFunction(mut f)) = env.lookup(&name) {
        return f.eval(&args);
    }

    return Err(anyhow!("{name} is not defined"));
}
fn eval_list(form: &Node<Form>, env: EnvPointer) -> Result<RuntimeObject> {
    eval_children(form, env).map(RuntimeObject::List)
}

fn eval_children(form: &Node<Form>, env: EnvPointer) -> Result<Vec<RuntimeObject>> {
    let a = form.deep_clone();
    a.iter()
        .map(|node| eval(node, env.clone()))
        .collect::<Result<Vec<RuntimeObject>>>()
}

fn eval_forest(forest: Forest<Form>, env: EnvPointer) -> Result<Vec<RuntimeObject>> {
    forest
        .iter()
        .map(|node| eval(node, env.clone()))
        .collect::<Result<Vec<RuntimeObject>>>()
}
pub fn eval_str(code: &str, env: EnvPointer) -> Result<RuntimeObject> {
    let mut ast = parse_string(code)?;
    let mut forest = ast.deep_clone_forest();
    let res = eval_forest(forest, env)?;
    Ok(res.last().unwrap().clone())
}
