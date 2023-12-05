use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day05::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);

    c.bench_function("lowest_location_backwards", |b| {
        b.iter(|| lowest_location_backwards(black_box(&input)))
    });

    c.bench_function("lowest_location_intervals", |b| {
        b.iter(|| lowest_location_intervals(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
