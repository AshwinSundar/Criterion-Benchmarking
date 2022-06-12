use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_benchmarking::{euler1_par, euler1_series, euler1_simple};

fn criterion_benchmark(c: &mut Criterion) {
    let input = 1000000;

    // Part 1 - intro

    // c.bench_function("simple", |b| b.iter(|| euler1_simple(black_box(input))));
    // c.bench_function("parallel", |b| b.iter(|| euler1_par(black_box(input))));
    // c.bench_function("series", |b| b.iter(|| euler1_series(black_box(input))));

    // Part 2 - groups

    // let mut group = c.benchmark_group("Euler 1");
    // group.bench_function( "simple", |b| b.iter(|| euler1_simple(black_box(input))) );
    // group.bench_function( "parallel" , |b| b.iter(|| euler1_par(black_box(input))));
    // group.bench_function( "series", |b| b.iter(|| euler1_series(black_box(input))));
    // group.finish();

    // Part 3 - multiple inputs
    // Would be cool to figure out how to build a RSM for 3 functions x inputs -> https://en.wikipedia.org/wiki/Response_surface_methodology

    let inputs = [100, 1000, 10000, 100000, 1000000];
    let mut group = c.benchmark_group("Multiple inputs");
    for i in inputs {
        group.bench_with_input(BenchmarkId::new("euler1_simple", i), &i, |b, &i| {
            b.iter(|| euler1_simple(black_box(i)))
        });
        group.bench_with_input(BenchmarkId::new("euler1_par", i), &i, |b, &i| {
            b.iter(|| euler1_par(black_box(i)))
        });
        group.bench_with_input(BenchmarkId::new("euler1_series", i), &i, |b, &i| {
            b.iter(|| euler1_series(black_box(i)))
        });
    }
    group.finish();

    // How does euler1_series perform at edge cases? e.g. 2^64 / 2
    // c.bench_function("series-maxi64", |b| b.iter(|| euler1_series(black_box(9 * i64::pow(10, 18)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);