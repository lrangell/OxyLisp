use criterion::{criterion_group, criterion_main, Criterion};
use oxy_lisp::env::init_env;
use oxy_lisp::evaluator::eval_str;

fn fibonacci(c: &mut Criterion) {
    let env = init_env();
    eval_str(
        "(defn fib [n] (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))",
        env.clone(),
    )
    .unwrap();

    c.bench_function("fib 10", |b| b.iter(|| eval_str("(fib 10)", env.clone())));
    c.bench_function("fib 17", |b| b.iter(|| eval_str("(fib 17)", env.clone())));
    c.bench_function("fib 20", |b| b.iter(|| eval_str("(fib 20)", env.clone())));
}

fn recursive_sum(c: &mut Criterion) {
    let mut env = init_env();
    eval_str(
        "(defn range-sum [limit acc] (if (= limit 0) acc (range-sum (+ limit -1) (+ acc limit))))",
        env.clone(),
    )
    .unwrap();

    c.bench_function("sum 100", |b| {
        b.iter(|| eval_str("(range-sum 100 0)", env.clone()))
    });
    c.bench_function("sum 1k", |b| {
        b.iter(|| eval_str("(range-sum 1000 0)", env.clone()))
    });
}
fn factorial(c: &mut Criterion) {
    let mut env = init_env();
    eval_str(
        "(defn fac [n] (if (= 1 n) 1 (+ n (fac (+ n -1)))))",
        env.clone(),
    )
    .unwrap();

    c.bench_function("factorial 100", |b| {
        b.iter(|| eval_str("(fac 100)", env.clone()))
    });
    c.bench_function("factorial 700", |b| {
        b.iter(|| eval_str("(fac 700)", env.clone()))
    });
}

criterion_group!(benches, fibonacci, recursive_sum, factorial);
criterion_main!(benches);
