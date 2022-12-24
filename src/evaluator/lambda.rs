use std::collections::HashMap;

use super::eval_children;
use crate::prelude::*;
use anyhow::*;
use trees::Tree;

impl Lambda {
    pub fn new(
        name: Option<String>,
        args: Vec<String>,
        body: Tree<Form>,
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
        self.env.vars.insert(
            self.name.to_owned().unwrap(),
            RuntimeObject::RuntimeFunction(self.to_owned()),
        );
    }
    pub fn eval(self: &mut Self, args: &[RuntimeObject]) -> Result<RuntimeObject> {
        // let remaing_args = self.args.len() - args.len();
        // if remaing_args > 1 {
        //     let mut partial_fn = self.clone();
        //     partial_fn.bind_symbols(args);
        //     partial_fn.args = self
        //         .args
        //         .clone()
        //         .into_iter()
        //         .skip(self.args.len() - remaing_args)
        //         .collect();
        //     return Ok(RuntimeObject::RuntimeFunction(partial_fn));
        // };

        self.bind_symbols(args);
        let result = eval_children(&mut self.body.root_mut(), &mut self.env)?
            .last()
            .unwrap_or(&RuntimeObject::NoOp)
            .to_owned();
        Ok(result)
    }
}

// TODO: implement into for types
