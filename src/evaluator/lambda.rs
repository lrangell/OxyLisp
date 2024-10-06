use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::eval_forest;
use crate::prelude::{display::PrintAST, *};
use anyhow::*;
use trees::{fr, tr, Node, Tree};

impl Lambda {
    pub fn new(name: Option<String>, args: Vec<String>, body: Tree<Form>, parent: Rc<Env>) -> Self {
        Lambda {
            name,
            args,
            body,
            self_recursive: false,
            env: Env {
                vars: RefCell::new(HashMap::new()),
                parent: Some(parent),
            },
        }
    }
    pub fn bind_symbols(&self, values: &[RuntimeObject]) {
        self.env.vars.borrow_mut().clear();

        self.env
            .vars
            .borrow_mut()
            .extend(self.args.iter().cloned().zip(values.iter().cloned()));
    }
    #[allow(dead_code)]
    pub fn print_body(&self) -> String {
        self.body.front().unwrap().print_ast().unwrap()
    }
    pub fn eval(&self, args: &[RuntimeObject]) -> Result<RuntimeObject> {
        self.bind_symbols(args);
        Ok(eval_forest(self.body.clone(), Rc::new(self.env.clone()))?
            .last()
            .cloned()
            .unwrap())
    }
}

#[allow(dead_code)]
fn lambda_cont(body: Tree<Form>) -> Tree<Form> {
    let lambda_arg = tr(Form::List) / tr(Form::Symbol("v".to_string()));
    let mut lambda = tr(Form::CallExpression("fn".to_string())) / lambda_arg;

    lambda.push_back(tr(Form::CallExpression("cont".to_string())) / body);
    lambda
}
#[allow(dead_code)]
fn find_recurr(root: &mut Node<Form>, name: String) -> Option<&mut Node<Form>> {
    unsafe {
        match root.data() {
            Form::CallExpression(sym) if *sym == name => Some(root),
            Form::CallExpression(_) => root
                .iter_mut()
                .find_map(|n| find_recurr(n.get_unchecked_mut(), name.clone())),
            _ => None,
        }
    }
}
