use advent_of_code_2024::{day1, day2, day3, day4, day5, day6};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("./input/2024/day1.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day1part1", |b| b.iter(|| day1::part1(black_box(input))));
    c.bench_function("day1part2", |b| b.iter(|| day1::part2(black_box(input))));

    let input = read_to_string("./input/2024/day2.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day2part1", |b| b.iter(|| day2::part1(black_box(input))));
    c.bench_function("day2part2", |b| b.iter(|| day2::part2(black_box(input))));

    let input = read_to_string("./input/2024/day3.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day3part1", |b| b.iter(|| day3::part1(black_box(input))));
    c.bench_function("day3part2", |b| b.iter(|| day3::part2(black_box(input))));

    let input = read_to_string("./input/2024/day4.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day4part1", |b| b.iter(|| day4::part1(black_box(input))));
    c.bench_function("day4part2", |b| b.iter(|| day4::part2(black_box(input))));

    let input = read_to_string("./input/2024/day5.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day5part1", |b| b.iter(|| day5::part1(black_box(input))));
    c.bench_function("day5part2", |b| b.iter(|| day5::part2(black_box(input))));

    let input = read_to_string("./input/2024/day6.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day6part1", |b| b.iter(|| day6::part1(black_box(input))));
    // c.bench_function("day6part2", |b| b.iter(|| day6::part2(black_box(input))));

    let input = read_to_string("./input/2024/day7.txt").unwrap();
    let input = input.as_str();
    c.bench_function("day7part1", |b| b.iter(|| day7::part1(black_box(input))));
    c.bench_function("day7part2", |b| b.iter(|| day7::part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
