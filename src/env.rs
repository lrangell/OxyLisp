use anyhow::{Context, Result};

use crate::types::*;

pub fn aritimetic() -> (String, BuiltinFn) {
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
    return ("+".to_string(), add.to_owned());
}
