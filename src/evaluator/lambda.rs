use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use super::{eval, eval_forest};
use crate::prelude::{display::PrintAST, *};
use anyhow::*;
// use log::debug;
use trees::{fr, tr, Forest, Node, Tree};

impl Lambda {
    pub fn new(
        name: Option<String>,
        args: Vec<String>,
        body: Forest<Form>,
        parent: Rc<Env>,
    ) -> Self {
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
    pub fn bind_symbols(&self, values: &[RuntimeObject]) -> () {
        self.env.vars.borrow_mut().clear();

        self.env
            .vars
            .borrow_mut()
            .extend(self.args.iter().cloned().zip(values.to_owned().into_iter()));
    }
    pub fn print_body(&self) -> String {
        self.body.front().unwrap().print_ast().unwrap()
    }
    pub fn eval(&self, args: &[RuntimeObject]) -> Result<RuntimeObject> {
        Ok(eval_forest(self.body.clone(), Rc::new(self.env.clone()))?
            .last()
            .cloned()
            .unwrap())
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
