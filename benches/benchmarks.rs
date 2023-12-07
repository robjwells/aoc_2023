use criterion::{criterion_group, criterion_main, Criterion};

extern crate aoc_2023;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1", |b| b.iter(aoc_2023::day01));
    c.bench_function("Day 2", |b| b.iter(aoc_2023::day02));
    c.bench_function("Day 3", |b| b.iter(aoc_2023::day03));
    c.bench_function("Day 4", |b| b.iter(aoc_2023::day04));
    c.bench_function("Day 5", |b| b.iter(aoc_2023::day05));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
