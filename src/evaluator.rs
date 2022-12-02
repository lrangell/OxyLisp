use crate::types::*;
use anyhow::Result;
use anyhow::*;
use std::collections::HashMap;

pub fn eval(exp: Expression, env: &Env) -> Result<Primitive> {
    let (toCall, forms) = exp;
    let nf = forms
        .iter()
        .map(|f| match f {
            Form::Primitive(p) => Ok(p.clone()),
            Form::Symbol(s) => eval_object(&env.vars.get(s).expect("444")),
            Form::Expression(e) => eval(e.clone(), env),
        })
        .collect::<Result<Vec<Primitive>>>()
        .expect("444");

    let cc = env.vars.get(&toCall).expect("ddd");

    match cc {
        Objects::BuiltinFn(f) => f(nf),
        Objects::Primitive(p) => Ok(p.clone()),
    }
}

fn eval_object(obj: &Objects) -> Result<Primitive> {
    let res = match obj {
        Objects::BuiltinFn(f) => None,
        Objects::Primitive(p) => Some(p.clone()),
    };
    Ok(res.ok_or("444").expect("444"))
}

// TODO: implement into for types
