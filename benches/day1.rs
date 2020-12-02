use criterion::{criterion_group, criterion_main, Criterion};

use adventofcode::day1;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| {
            let lines = day1::get_input();
            day1::p1(&lines)
        })
    });

    c.bench_function("part 2", |b| {
        b.iter(|| {
            let lines = day1::get_input();
            day1::p2(&lines)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
