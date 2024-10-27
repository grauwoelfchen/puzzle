#[macro_use]
extern crate criterion;

#[path = "../src/main.rs"]
mod puzzle;

use criterion::{black_box, Criterion};
use puzzle::Solution;

fn bench_fixme(c: &mut Criterion) {
    c.bench_function("Benchmark", |b| {
        b.iter(|| {
            let n = black_box(1000);
            Solution::fixme(n)
        })
    });
}

criterion_group!(bench, bench_fixme);
criterion_main!(bench);
