use criterion::{criterion_group, BenchmarkId, Criterion};

use adventofcode::day2;

pub fn compare(c: &mut Criterion) {
    let input = day2::get_input();

    c.bench_with_input(BenchmarkId::new("day2", "1"), &input, |b, input| {
        let input = input.to_vec();
        b.iter(|| {
            let lines = day2::parse_lines(&input);
            day2::p1(lines)
        })
    });

    c.bench_with_input(BenchmarkId::new("day2", "2"), &input, |b, input| {
        let input = input.to_vec();
        b.iter(|| {
            let lines = day2::parse_lines(&input);
            day2::p2(lines)
        })
    });
}

criterion_group!(day2, compare);
