use serial_test::serial;

use crate::{parser::*, types::display::PrintAST};
use pretty_assertions::assert_eq;

fn assert_parse_eq(code: &str) {
    let ast = parse_string(code).unwrap();
    assert_eq!(code, ast.root().print_ast().unwrap());
}
#[test]
#[serial]
fn arithmetic() {
    assert_parse_eq("(+ 10 90 7 3)");
    assert_parse_eq("(* (+ 4 4) 90 3)")
}

#[test]
#[serial]
fn call_expressions() {
    assert_parse_eq("(def sym 1000)");
    assert_parse_eq("(def var (+ 4 3))");
    assert_parse_eq("(defn inc [x] (+ 1 x)) (inc 10)");
}

#[test]
fn lists() {
    assert_parse_eq("[1 2 3 4] (+ 1 2)");
    assert_parse_eq("(def l [1 2 3 (+ 4 2)])")
}
