use crate::{env::init_env, evaluator::eval_forms_vec, lexer::tokenize, parser::*};
use serial_test::serial;
use test_env_log::test;

fn eval_and_assert_eq(code: &str, val: &str) {
    let mut env = init_env();
    let forms = parse(&tokenize(code)).unwrap();
    println!("{:?}", forms);
    let vals = eval_forms_vec(&forms, &mut env).unwrap();
    assert_eq!(vals.last().unwrap().to_string(), val);
}

#[test]
#[serial]
fn defn() {
    eval_and_assert_eq("(defn id [x] x)  (id [1 2 3])", "[1 2 3]");
    eval_and_assert_eq("(defn inc [x] (+ 1 x))  (inc 10)", "11");
}
