use crate::prelude::*;
use anyhow::{Context, Result};
use std::collections::hash_map;

pub fn aritimetic() -> [(String, RuntimeObject); 2] {
    let add: BuiltinFn = |nums: Vec<Literal>| -> Result<Literal> {
        nums.iter()
            .filter_map(|s| match s {
                Literal::Integer(i) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc + curr)
            .map(Literal::Integer)
            .context("All arguments must be integers")
    };
    let mult: BuiltinFn = |nums: Vec<Literal>| -> Result<Literal> {
        nums.iter()
            .filter_map(|s| match s {
                Literal::Integer(i) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc * curr)
            .map(Literal::Integer)
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
