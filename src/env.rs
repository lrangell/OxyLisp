use crate::prelude::*;
use log::debug;
use std::collections::hash_map;

pub fn init_env() -> Box<Env> {
    Box::new(Env {
        vars: hash_map::HashMap::new(),
        parent: EnvType::RootEnv,
    })
}

impl Env {
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
