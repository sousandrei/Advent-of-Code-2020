use criterion::{criterion_group, BenchmarkId, Criterion};

use adventofcode::day1;

pub fn compare(c: &mut Criterion) {
    let lines = day1::get_input();

    c.bench_with_input(BenchmarkId::new("day1", "1"), &lines.clone(), |b, lines| {
        b.iter(|| day1::p1(lines))
    });

    c.bench_with_input(BenchmarkId::new("day1", "2"), &lines.clone(), |b, lines| {
        b.iter(|| day1::p2(lines))
    });
}

criterion_group!(day1, compare);
