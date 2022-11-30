use std::collections::hash_map;

use anyhow::{Context, Result};

use crate::types::*;

pub fn aritimetic() -> (String, Objects) {
    let add: BuiltinFn = |nums: &[Primitive]| -> Result<Primitive> {
        nums.iter()
            .filter_map(|s| match s {
                Primitive::Integer(i) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc + curr)
            .map(Primitive::Integer)
            .context("All arguments must be integers")
    };
    return ("+".to_string(), Objects::BuiltinFn(add));
}

pub fn init_env() -> Env {
    return Env {
        vars: hash_map::HashMap::from([aritimetic()]),
    };
}
