use std::{borrow::BorrowMut, collections::HashMap};

use super::{eval_children, eval_forest};
use crate::prelude::{display::PrintAST, *};
use anyhow::*;
use log::debug;
use trees::{tr, tree, Forest, Node, Tree, TreeWalk};

impl Lambda {
    pub fn new(
        name: Option<String>,
        args: Vec<String>,
        body: Forest<Form>,
        parent: Box<Env>,
    ) -> Self {
        Lambda {
            name,
            args,
            body,
            env: Box::new(Env {
                vars: HashMap::new(),
                parent: EnvType::LambdaEnv(parent),
            }),
        }
    }
    pub fn bind_symbols(self: &mut Self, values: &[RuntimeObject]) -> () {
        //TODO: check for arity
        self.env
            .vars
            .extend(self.args.iter().cloned().zip(values.to_owned().into_iter()));
        match &self.name {
            Some(name) => {
                self.env.vars.insert(
                    name.to_string(),
                    RuntimeObject::RuntimeFunction(self.to_owned()),
                );
            }
            None => {}
        }
    }
    pub fn eval(self: &mut Self, args: &[RuntimeObject]) -> Result<RuntimeObject> {
        self.bind_symbols(args);
        let result = eval_forest(&mut self.body, &mut self.env)?
            .last()
            .unwrap_or(&RuntimeObject::NoOp)
            .to_owned();
        Ok(result)
    }
}
