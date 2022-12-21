#![feature(iter_advance_by)]

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
    repl::init()
}
