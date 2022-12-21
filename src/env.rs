use crate::prelude::*;
use anyhow::*;
use std::collections::hash_map;

pub fn aritimetic() -> [(String, RuntimeObject); 3] {
    let add: BuiltinFn = |nums: &[Form], _| -> Result<Form> {
        nums.iter()
            .filter_map(|s| match s {
                Form::Literal(Literal::Integer(i)) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc + curr)
            .map(|v| v.into())
            .context("44")
    };
    let mult: BuiltinFn = |nums: &[Form], _| -> Result<Form> {
        nums.iter()
            .filter_map(|s| match s {
                Form::Literal(Literal::Integer(i)) => Some(*i),
                _ => None,
            })
            .reduce(|acc, curr| acc * curr)
            .map(|v| v.into())
            .context("All arguments must be integers")
    };
    let less: BuiltinFn = |nums: &[Form], _| -> Result<Form> {
        match nums {
            [Form::Literal(Literal::Integer(left)), Form::Literal(Literal::Integer(right))] => {
                Ok(Form::Literal(Literal::Bool(left < right)))
            }
            _ => Err(anyhow!("Arguments must be numbers")),
        }
    };
    return [
        ("+".to_string(), RuntimeObject::Function(add)),
        ("*".to_string(), RuntimeObject::Function(mult)),
        ("<".to_string(), RuntimeObject::Function(less)),
    ];
}

pub fn defs() -> [(String, RuntimeObject); 1] {
    let def: BuiltinFn = |forms: &[Form], env: &mut Env| -> Result<Form> {
        match forms {
            [Form::Symbol(symbol), Form::Literal(value)] => {
                env.vars
                    .insert(symbol.to_string(), RuntimeObject::Primitive(value.clone()));
                Ok(Literal::Bool(true).into())
            }
            _ => Err(anyhow!("err")),
        }
    };
    [("def".to_string(), RuntimeObject::Function(def))]
}

pub fn init_env() -> Box<Env> {
    // let funs = [&defs()[..], &aritimetic()[..]].concat().to_owned();
    let funs: Vec<(String, RuntimeObject)> =
        defs().iter().chain(aritimetic().iter()).cloned().collect();
    let mut vars: std::collections::HashMap<String, RuntimeObject> = hash_map::HashMap::new();
    for (k, v) in funs {
        vars.insert(k, v);
    }

    Box::new(Env {
        vars,
        parent: EnvType::RootEnv,
    })
}
