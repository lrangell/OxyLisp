use crate::prelude::*;
use anyhow::*;
use log::debug;
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

impl Env {
    pub fn def(&mut self, symbol: &Form, value: &RuntimeObject) -> Result<RuntimeObject> {
        let Form::Symbol(sym) = symbol else {
            return Err(anyhow!("First argument of def must be a symbol"))
        };
        // debug!("def sym: {} val: {}", sym, value);
        self.vars.insert(sym.clone(), value.clone());
        Ok(RuntimeObject::Primitive(Literal::Bool(true)))
    }
    pub fn defn(
        self: &mut Box<Self>,
        symbol: &Form,
        arguments: Vec<String>,
        forms: Vec<Form>,
    ) -> Result<RuntimeObject> {
        let Form::Symbol(sym) = symbol else {
            return Err(anyhow!("First argument of defn must be a symbol"))
        };
        let function = Lambda::new(Some(symbol.to_string()), arguments, forms, self.to_owned());
        self.vars
            .insert(sym.to_string(), RuntimeObject::RuntimeFunction(function));

        debug!("Function {sym} defined");
        Ok(Literal::Nil.into())
    }

    pub fn lookup(&self, symbol: &str) -> Option<RuntimeObject> {
        debug!("Lookup: {symbol} ");
        if let Some(value) = self.vars.get(symbol) {
            debug!("found: {symbol} ");
            return Some(value.clone());
        }
        match &self.parent {
            EnvType::RootEnv => {
                debug!("root lookup end");
                None
            }
            EnvType::LambdaEnv(parent) => {
                debug!("lookup parent");
                parent.lookup(symbol)
            }
        }
    }
}
