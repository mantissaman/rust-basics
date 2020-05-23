use criterion::{black_box, criterion_group, criterion_main, Criterion};
use m007_criterion::{fast_fibonacci, slow_fibonacci};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci slow", |b| b.iter(|| slow_fibonacci(8)));
}
fn fibonacci_benchmark2(c: &mut Criterion) {
    c.bench_function("fibonacci fast", |b| b.iter(|| fast_fibonacci(8)));
}


criterion_group!(fib_bench, fibonacci_benchmark, fibonacci_benchmark2);
criterion_main!(fib_bench);