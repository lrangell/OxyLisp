use crate::prelude::*;
use anyhow::{Context, Ok, Result};
use std::collections::hash_map;

pub fn aritimetic() -> [(String, Objects); 2] {
    let add: BuiltinFn = |nums: Vec<Primitive>| -> Result<Primitive> {
        nums.iter()
            .filter_map(|s| match s {
                Primitive::Integer(i) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc + curr)
            .map(Primitive::Integer)
            .context("All arguments must be integers")
    };
    let mult: BuiltinFn = |nums: Vec<Primitive>| -> Result<Primitive> {
        nums.iter()
            .filter_map(|s| match s {
                Primitive::Integer(i) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc * curr)
            .map(Primitive::Integer)
            .context("All arguments must be integers")
    };
    return [
        ("+".to_string(), Objects::BuiltinFn(add)),
        ("*".to_string(), Objects::BuiltinFn(mult)),
    ];
}

pub fn init_env() -> Env {
    return Env {
        vars: hash_map::HashMap::from(aritimetic()),
    };
}
