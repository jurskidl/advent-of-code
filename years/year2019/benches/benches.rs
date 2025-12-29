use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;
// use year2019::day1;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("./input/2019/day1.txt").unwrap();
    let input = input.as_str();
    // c.bench_function("day1part1", |b| b.iter(|| day1::part1(black_box(input))));
    // c.bench_function("day1part2", |b| b.iter(|| day1::part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
