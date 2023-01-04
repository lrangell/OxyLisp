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
fn fold(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    if let [RuntimeObject::RuntimeFunction(ff), RuntimeObject::List(list), init] = args {
        return list.iter().fold(
            Ok(init.clone()),
            |acc: Result<RuntimeObject>, curr: &RuntimeObject| -> Result<RuntimeObject> {
                ff.eval(&[acc?, curr.clone()])
            },
        );
    };
    Err(anyhow!("Invalid arguments to fold"))
}

fn add(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let sum: i32 = args.extract_numbers()?.iter().sum();
    Ok(sum.into())
}
fn subtract(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let num_args = args.extract_numbers()?;
    let (head, nums) = num_args.split_first().unwrap();
    let res: i32 = *head - nums.to_vec().iter().sum::<i32>();
    Ok(res.into())
}

fn multiply(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let product: i32 = args.extract_numbers()?.iter().product();
    Ok(product.into())
}
fn less(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let [left, right] = args.extract_numbers()?[..] else {
          return Err(anyhow!("arguments must be numbers"));
    };
    Ok((left < right).into())
}
fn equal(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let rtos = args.to_owned();
    let [RuntimeObject::Primitive(left), RuntimeObject::Primitive(right)] = rtos.as_slice() else {
        return Err(anyhow!(""));
    };
    Ok((left == right).into())
}
fn fold_or(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let res = args
        .extract_bools()?
        .into_iter()
        .reduce(|curr, acc| acc || curr)
        .unwrap();
    Ok(res.into())
}
fn concat(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let list = args.extract_lists()?.into_iter().flatten().collect();
    Ok(RuntimeObject::List(list))
}
fn range(args: &[RuntimeObject]) -> Result<RuntimeObject> {
    let args_as_numbers = args.extract_numbers()?;
    let [start, end] = args_as_numbers[..] else {return Err(anyhow!("Range need start and end"));};
    let list: Vec<RuntimeObject> = (start..end)
        .map(|i| RuntimeObject::Primitive(Literal::Integer(i)))
        .collect();

    Ok(RuntimeObject::List(list))
}

static MAP: [(&str, BuiltInFunction); 10] = [
    ("fold", fold),
    ("map", map),
    ("+", add),
    ("-", subtract),
    ("*", multiply),
    ("<", less),
    ("=", equal),
    ("or", fold_or),
    ("concat", concat),
    ("range", range),
];

pub static FUNCTIONS: Lazy<HashMap<&str, BuiltInFunction>> = Lazy::new(|| HashMap::from(MAP));
