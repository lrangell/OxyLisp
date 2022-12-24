use crate::prelude::*;
use oxy_lisp::{env::init_env, evaluator::eval_str};
use serial_test::serial;
use test_env_log::test;

fn eval_and_assert_eq(code: &str, val: &str) {
    let mut env = init_env();
    let res = eval_str(code, &mut env).unwrap();
    // println!("{:?}", forms);
    // let vals = eval_forms_vec(&forms, &mut env).unwrap();
    assert_eq!(res.to_string(), val);
}

#[test]
#[serial]
fn defn() {
    eval_and_assert_eq("(defn id [x] x)  (id [1 2 3])", "[1 2 3]");
    eval_and_assert_eq("(defn inc [x] (+ 1 x))  (inc 10)", "11");
    eval_and_assert_eq(
        "(defn fib [n] (if (< n 2) n (+ (fib (+ n -1)) (fib (+ n -2))))) (fib 15)",
        "610",
    );
    eval_and_assert_eq(
        "(defn id [x] x) (defn inc [x] (+ 1 x)) (defn comp [x] (inc (id x))) (comp 10)",
        "11",
    );
}
