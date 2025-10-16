#![allow(unused)]
use criterion::{criterion_group, criterion_main, Criterion};

fn dummy_benchmark(c: &mut Criterion) {
    c.bench_function("noop", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, dummy_benchmark);
criterion_main!(benches);
#![allow(unused)]
#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_placeholder(b: &mut Bencher) {
    b.iter(|| 1 + 1);
}
