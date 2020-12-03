use criterion::{criterion_group, BenchmarkId, Criterion};

use adventofcode::day3;

pub fn compare(c: &mut Criterion) {
    let input = day3::get_input();

    c.bench_with_input(BenchmarkId::new("day3", "1"), &input, |b, input| {
        b.iter(|| day3::p1(input.clone()))
    });

    c.bench_with_input(BenchmarkId::new("day3", "2"), &input, |b, input| {
        b.iter(|| day3::p2(input.clone()))
    });
}

criterion_group!(day3, compare);
