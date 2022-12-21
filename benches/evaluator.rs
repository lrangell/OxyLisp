use criterion::{criterion_group, criterion_main, Criterion};
use oxy_lisp::env::init_env;
use oxy_lisp::evaluator::eval_from_str;

fn criterion_benchmark(c: &mut Criterion) {
    let mut env = init_env();
    eval_from_str(
        "(defn fib [n] (if (< n 2) n (+ (fib (+ n -1)) (fib (+ n -2)))))",
        &mut env,
    )
    .unwrap();

    c.bench_function("fib 15", |b| b.iter(|| eval_from_str("(fib 15)", &mut env)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
