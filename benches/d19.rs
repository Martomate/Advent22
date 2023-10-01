use std::time::Duration;

use advent22::days::d19;

use criterion::{criterion_group, criterion_main, Criterion};

fn small_example() -> Vec<String> {
    include_str!("../src/days/d19/ex1.txt")
        .split('\n')
        .map(|s| s.to_owned())
        .collect()
}

fn big_example() -> Vec<String> {
    include_str!("../src/days/d19/ex2.txt")
        .split('\n')
        .map(|s| s.to_owned())
        .collect()
}

fn bench_mini(c: &mut Criterion) {
    let mut group = c.benchmark_group("mini");
    group.warm_up_time(Duration::from_millis(1));
    group.sample_size(100);
    group.bench_function("small_example", |b| {
        b.iter(|| d19::run_program(small_example(), 4, true))
    });
}

fn bench_small_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("small");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(10);
    group.bench_function("small_example", |b| {
        b.iter(|| d19::run_program(small_example(), 24, true))
    });
}

fn bench_big_example_mini(c: &mut Criterion) {
    let mut group = c.benchmark_group("mini");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(100);
    group.bench_function("big_example", |b| {
        b.iter(|| d19::run_program(big_example(), 4, true))
    });
}

fn bench_big_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("small");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(10);
    group.bench_function("big_example", |b| {
        b.iter(|| d19::run_program(big_example(), 24, true))
    });
}

criterion_group!(benches, bench_mini, bench_small_example, bench_big_example_mini); //, bench_big_example);
criterion_main!(benches);
