use crate::prelude::*;
use anyhow::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn map(args: &[RuntimeObject], _env: &mut Box<Env>) -> Result<RuntimeObject, Error> {
    if let [RuntimeObject::RuntimeFunction(f), RuntimeObject::List(list)] = args {
        return list
            .iter()
            .map(|el| f.to_owned().eval(&[el.to_owned()]))
            .collect::<Result<Vec<RuntimeObject>>>()
            .map(RuntimeObject::List);
    };
    Err(anyhow!(" "))
}

fn add(args: &[RuntimeObject], _env: &mut Box<Env>) -> Result<RuntimeObject> {
    let sum = args.extract_numbers()?.iter().sum();
    Ok(RuntimeObject::Primitive(Literal::Integer(sum)))
}
fn multiply(args: &[RuntimeObject], _env: &mut Box<Env>) -> Result<RuntimeObject> {
    let product = args.extract_numbers()?.iter().product();
    Ok(RuntimeObject::Primitive(Literal::Integer(product)))
}
fn less(args: &[RuntimeObject], _env: &mut Box<Env>) -> Result<RuntimeObject> {
    let [left, right] = args.extract_numbers()?[..] else {
          return Err(anyhow!("arguments must be numbers"));
    };
    Ok(RuntimeObject::Primitive(Literal::Bool(left < right)))
}
fn equal(args: &[RuntimeObject], _env: &mut Box<Env>) -> Result<RuntimeObject> {
    let rtos = args.to_owned();
    let [RuntimeObject::Primitive(left), RuntimeObject::Primitive(right)] = rtos.as_slice() else {
        return Err(anyhow!(""));
    };
    Ok(RuntimeObject::Primitive(Literal::Bool(left == right)))
}

static MAP: [(&str, BuiltInFunction); 5] = [
    ("map", map),
    ("+", add),
    ("*", multiply),
    ("<", less),
    ("=", equal),
];

pub static FUNCTIONS: Lazy<HashMap<&str, BuiltInFunction>> = Lazy::new(|| HashMap::from(MAP));
