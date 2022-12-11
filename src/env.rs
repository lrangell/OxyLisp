use crate::prelude::*;
use anyhow::*;
use std::collections::hash_map;

pub fn aritimetic() -> [(String, RuntimeObject); 2] {
    let add: BuiltinFn = |nums: &[Form]| -> Result<Form> {
        nums.iter()
            .filter_map(|s| match s {
                Form::Literal(Literal::Integer(i)) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc + curr)
            .map(|v| v.into())
            .context("44")

        // .ok_or(Err("44444").into())
        // .ok_or("dddd")
        // .context("All arguments must be integers")
    };
    let mult: BuiltinFn = |nums: &[Form]| -> Result<Form> {
        nums.iter()
            .filter_map(|s| match s {
                Form::Literal(Literal::Integer(i)) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc * curr)
            .map(|v| v.into())
            .context("All arguments must be integers")
    };
    return [
        ("+".to_string(), RuntimeObject::Function(add)),
        ("*".to_string(), RuntimeObject::Function(mult)),
    ];
}

pub fn init_env() -> Env {
    return Env {
        vars: hash_map::HashMap::from(aritimetic()),
    };
}
