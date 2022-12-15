#![feature(iter_advance_by)]

#[macro_use]
extern crate log;

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
    env_logger::init();
    let defcode = "(def xx (+ 50 (* 25 2)) )";
    // let addcode = "(+ 1 (* 10 2) 2 100)";
    let addcode = "(+ xx (* 10 2) )";

    let mut basic_env = env::init_env();
    let __r = evaluator::eval_from_str(defcode, &mut basic_env);
    let r = evaluator::eval_from_str(addcode, &mut basic_env);
    println!("{:?}", r);
}
