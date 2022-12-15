use crate::{lexer::tokenize, parser::*};
use anyhow::*;

fn eval_and_assert_eq(code: &str) {
    let parsed_code = parse(&tokenize(code)).unwrap().first().unwrap().to_string();
    assert_eq!(code, parsed_code);
    println!("parsed: {}", parsed_code);
}
#[test]
fn arithmetic() {
    eval_and_assert_eq("(+ 10 90 7 3)");
    eval_and_assert_eq("(* (+ 4 0 9 8 8 8 4) 90 7 3)")
}

#[test]
fn call_expressions() {
    eval_and_assert_eq("(def sym 1000)");
    eval_and_assert_eq("(def var (+ 4 3))");
}

#[test]
fn lists() {
    eval_and_assert_eq("[1 2 3 4");
    eval_and_assert_eq("(def l [1 2 3 (+ 4 2)])")
}
