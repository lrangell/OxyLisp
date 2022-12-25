use std::{borrow::BorrowMut, collections::HashMap, pin::Pin};

use super::{eval_children, eval_forest};
use crate::prelude::{display::PrintAST, *};
use anyhow::*;
use log::debug;
use trees::{fr, node, tr, tree, Forest, Node, Tree, TreeWalk};

impl Lambda {
    pub fn new(
        name: Option<String>,
        args: Vec<String>,
        body: Forest<Form>,
        parent: Box<Env>,
    ) -> Self {
        if let Some(fname) = name.clone() {
            let cps_body = cps(&mut body.clone().back_mut().unwrap(), fname);
            let mut argss = args.clone();
            argss.push("cont".to_string());
            return Lambda {
                name,
                args: argss,
                body: cps_body,
                env: Box::new(Env {
                    vars: HashMap::new(),
                    parent: EnvType::LambdaEnv(parent),
                }),
            };
        }
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

fn lambda_cont(body: Tree<Form>) -> Tree<Form> {
    let lambda_arg = tr(Form::List) / tr(Form::Symbol("v".to_string()));
    let mut lambda = tr(Form::CallExpression("fn".to_string())) / lambda_arg;

    lambda.push_back(tr(Form::CallExpression("cont".to_string())) / body);
    lambda
}

fn find_recurr(root: &mut Node<Form>, name: String) -> Option<&mut Node<Form>> {
    unsafe {
        match root.data() {
            Form::CallExpression(sym) if *sym == name => Some(root),
            Form::CallExpression(_) => root
                .iter_mut()
                .find_map(|mut n| find_recurr(n.get_unchecked_mut(), name.clone())),
            _ => None,
        }
    }
}

fn rearrange_nodes(root: &mut Node<Form>, name: String) -> Tree<Form> {
    if let Form::CallExpression(_) = root.data() {
        let mut recurr_node = find_recurr(root, name).unwrap();

        recurr_node.insert_next_sib(tr(Form::Symbol("v".to_string())));

        //
        let mut new_form = recurr_node.detach();
        new_form.push_back(lambda_cont(root.deep_clone()));
        return new_form;
    }
    tr(Form::CallExpression("cont".to_string())) / root.deep_clone()
}
fn cps(root: &mut Node<Form>, name: String) -> Forest<Form> {
    match root.data() {
        Form::CallExpression(sym) if sym.to_string() == "if" => {
            let condition = root.pop_front().unwrap();
            let mut left_forest = root.pop_front().unwrap().deep_clone();
            let mut right_forest = root.pop_front().unwrap().deep_clone();
            let mut left = left_forest.root_mut();
            let mut right = right_forest.root_mut();
            let cps_left = rearrange_nodes(&mut left, name.clone());
            let cps_right = rearrange_nodes(&mut right, name);
            let mut new_tree = tr(Form::CallExpression("if".to_string()));
            new_tree.push_back(condition);
            new_tree.push_back(cps_left);
            new_tree.push_back(cps_right);
            let mut forest: Forest<Form> = fr();
            forest.push_front(new_tree);
            forest
        }
        Form::CallExpression(_) => rearrange_nodes(root, name).deep_clone_forest(),
        _ => root.deep_clone_forest(),
    }
}
