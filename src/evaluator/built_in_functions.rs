use crate::prelude::*;
use anyhow::*;
use once_cell::sync::Lazy;
use std::{collections::HashMap, rc::Rc};

fn map(args: &[RuntimeObject]) -> Result<RuntimeObject, Error> {
    if let [RuntimeObject::RuntimeFunction(f), RuntimeObject::List(list)] = args {
        return list
            .iter()
            .map(|el| f.to_owned().eval(&[el.to_owned()]))
            .collect::<Result<Vec<RuntimeObject>>>()
            .map(RuntimeObject::List);
    };
    Err(anyhow!(" "))
}

fn add(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let sum = args.extract_numbers()?.iter().sum();

    Ok(RuntimeObject::Primitive(Literal::Integer(sum)))
}
fn subtract(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let num_args = args.extract_numbers()?;
    let (head, nums) = num_args.split_first().unwrap();
    let res: i32 = *head - nums.to_vec().iter().sum::<i32>();
    Ok(RuntimeObject::Primitive(Literal::Integer(res)))
}

fn multiply(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let product = args.extract_numbers()?.iter().product();
    Ok(RuntimeObject::Primitive(Literal::Integer(product)))
}
fn less(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let [left, right] = args.extract_numbers()?[..] else {
          return Err(anyhow!("arguments must be numbers"));
    };
    Ok(RuntimeObject::Primitive(Literal::Bool(left < right)))
}
fn equal(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let rtos = args.to_owned();
    let [RuntimeObject::Primitive(left), RuntimeObject::Primitive(right)] = rtos.as_slice() else {
        return Err(anyhow!(""));
    };
    Ok(RuntimeObject::Primitive(Literal::Bool(left == right)))
}
fn fold_or(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let res = args
        .extract_bools()?
        .into_iter()
        .reduce(|curr, acc| acc || curr)
        .unwrap();
    Ok(RuntimeObject::Primitive(Literal::Bool(res)))
}

static MAP: [(&str, BuiltInFunction); 7] = [
    ("map", map),
    ("+", add),
    ("-", subtract),
    ("*", multiply),
    ("<", less),
    ("=", equal),
    ("or", fold_or),
];

pub static FUNCTIONS: Lazy<HashMap<&str, BuiltInFunction>> = Lazy::new(|| HashMap::from(MAP));
