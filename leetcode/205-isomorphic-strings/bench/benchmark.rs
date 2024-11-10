// #![feature(test)]
// extern crate test as test_std;

#[macro_use]
extern crate criterion;

#[path = "../src/main.rs"]
mod puzzle;

use criterion::{black_box, Criterion};
use puzzle::Solution;

// use test_std::{Bencher, black_box};

// #[bench]
// fn bench_fixme(b: &mut Bencher) {
//     b.iter(|| {
//         let n = black_box(1000);
//         Solution::fixme(n)
//     })
// }

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
