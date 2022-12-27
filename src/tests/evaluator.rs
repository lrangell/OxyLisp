use crate::prelude::*;
use oxy_lisp::{env::init_env, evaluator::eval_str};
use serial_test::serial;
use test_env_log::test;

fn eval_and_assert_eq(code: &str, val: &str) {
    let env = init_env();
    let res = eval_str(code, env).unwrap();
    assert_eq!(res.to_string(), val);
}

// #[test]
// #[serial]
fn defn() {
    eval_and_assert_eq("(defn id [x] x)  (id [1 2 3])", "[1 2 3]");
    eval_and_assert_eq("(defn inc [x] (+ 1 x))  (inc 10)", "11");
    eval_and_assert_eq(
        "(defn fib [x] (if (< x 3) x (+ (fib (- x 1)) (fib (- x 1))))) (fib 10)",
        "610",
    );
    eval_and_assert_eq(
        "(defn id [x] x) (defn inc [x] (+ 1 x)) (defn comp [x] (inc (id x))) (comp 10)",
        "11",
    );
}
#[test]
#[serial]
fn fibonnaci() {
    eval_and_assert_eq(
        "(defn fib [x] (if (or (= 0 x) (= 1 x)) x (+ (fib (- x 1)) (fib (- x 2))))) (fib 10)",
        "55",
    )
}

#[test]
#[serial]
fn factorial() {
    eval_and_assert_eq(
        "(defn fac [n] (if (< n 2) n (* n (fac (- n 1))))) (fac 10)",
        "3628800",
    );
    eval_and_assert_eq(
        "(defn fac [n] (if (< n 2) n (* n (fac (- n 1))))) (fac 12)",
        "479001600",
    )
}
