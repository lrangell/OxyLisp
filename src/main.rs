#![feature(iter_advance_by)]
use crate::types::{codeChunk::CodeChunk, Literal};
#[macro_use]
extern crate log;

mod env;
mod evaluator;
mod lexer;
mod parser;
mod prelude;
mod types;

fn main() {
    env_logger::init();
    // std::env::set_var("RUST_LOG", "trace");
    let a = Literal::Bool(true);
    let b = Literal::Integer(4444);
    // let addcode = "(+ 1 (* 10 (+ 1 1)) 2)";
    let addcode = "(+ 1 (* 10 2) 2 100)";
    let code = CodeChunk::new(addcode);
    // let code2 = CodeChunk::new(addcode);
    // let cvec: Vec<types::Form> = code2.into_iter().collect();
    // println!("cvec: {:?}", cvec);

    let basic_env = env::init_env();
    let res = evaluator::eval(code, &basic_env);
    debug!("debug");
    println!("Hello, world! {} {}", a, b);
    println!("res: {:?}", res);
}
