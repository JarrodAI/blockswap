#![allow(unused)]
use criterion::{criterion_group, criterion_main, Criterion};

fn consensus_mock(c: &mut Criterion) {
    c.bench_function("consensus_mock", |b| b.iter(|| 42));
}

criterion_group!(benches, consensus_mock);
criterion_main!(benches);
#![allow(unused)]
#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_consensus_placeholder(b: &mut Bencher) {
    b.iter(|| 2 + 2);
}
