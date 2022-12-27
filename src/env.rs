use crate::prelude::*;
use log::debug;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::{hash_map, HashMap},
    rc::Rc,
};

pub fn init_env() -> EnvPointer {
    Rc::new(Env {
        vars: RefCell::new(HashMap::new()),
        parent: None,
    })
}

impl Env {
    pub fn lookup(&self, symbol: &str) -> Option<RuntimeObject> {
        debug!("lookup: {symbol} env: {:?}", self.defined_symbols());
        if let Some(value) = self.vars.borrow().get(symbol) {
            debug!("found: {symbol} ");
            return Some(value.clone());
        }
        match &self.parent {
            Some(parent) => parent.lookup(symbol),
            None => self.vars.borrow().get(symbol).cloned(),
        }
    }
    pub fn defined_symbols(&self) -> Vec<String> {
        self.vars
            .borrow()
            .clone()
            .into_keys()
            .collect::<Vec<String>>()
    }
}
