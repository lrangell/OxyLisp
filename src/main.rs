#![feature(iter_advance_by)]
#![feature(box_into_inner)]
extern crate lazy_static;

use std::env::args;

extern crate log;

extern crate trees;

mod env;
mod evaluator;
mod lexer;
mod parser;
mod prelude;
mod repl;
#[cfg(test)]
mod tests;
mod types;

fn main() {
    let a = args().collect::<Vec<String>>();
    if a.len() < 2 {
        repl::init();
    } else {
        let env = env::init_env();
        let r = evaluator::eval_str(&a[1], env);
        println!("{}", r.unwrap());
    }
}
