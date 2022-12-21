use crate::prelude::*;
use anyhow::*;
use std::collections::hash_map;

pub fn aritimetic() -> [(String, RuntimeObject); 3] {
    let add: BuiltinFn = |rts: &[RuntimeObject], _| -> Result<RuntimeObject> {
        let sum = rts.extract_numbers()?.iter().sum();
        Ok(RuntimeObject::Primitive(Literal::Integer(sum)))
    };
    let mult: BuiltinFn = |rts: &[RuntimeObject], _| -> Result<RuntimeObject> {
        let product = rts.extract_numbers()?.iter().product();
        Ok(RuntimeObject::Primitive(Literal::Integer(product)))
    };
    let less: BuiltinFn = |rts: &[RuntimeObject], _| -> Result<RuntimeObject> {
        let [left, right] = rts.extract_numbers()?[..] else {
            return Err(anyhow!("arguments must be numbers"));
        };
        Ok(RuntimeObject::Primitive(Literal::Bool(left < right)))
    };
    return [
        ("+".to_string(), RuntimeObject::Function(add)),
        ("*".to_string(), RuntimeObject::Function(mult)),
        ("<".to_string(), RuntimeObject::Function(less)),
    ];
}

pub fn init_env() -> Box<Env> {
    Box::new(Env {
        vars: hash_map::HashMap::from(aritimetic()),
        parent: EnvType::RootEnv,
    })
}
