use serial_test::serial;

use crate::{lexer::tokenize, parser::*};

fn assert_parse_eq(code: &str) {
    let parsed_code: Vec<String> = parse(&tokenize(code))
        .unwrap()
        .iter()
        .map(|f| f.to_string())
        .collect();
    assert_eq!(code, parsed_code.join(" "));
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
