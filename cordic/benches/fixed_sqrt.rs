use cordic::sqrt;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fixed::types::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("std");
    g.bench_function("f32 sqrt 2", |b| {
        b.iter(|| {
            let _s = black_box(2f32).sqrt();
        })
    });
    g.bench_function("f32 sqrt i32::MAX/2+2", |b| {
        b.iter(|| {
            let _s = black_box(i32::MAX as f32 / 2f32 + 2f32).sqrt();
        })
    });
    g.bench_function("f64 sqrt 2", |b| {
        b.iter(|| {
            let _s = black_box(2f64).sqrt();
        })
    });
    g.bench_function("f64 sqrt i64::MAX/2+2", |b| {
        b.iter(|| {
            let _s = black_box(i64::MAX as f64 / 2f64 + 2f64).sqrt();
        })
    });
    g.finish();

    let mut g = c.benchmark_group("fixed");
    g.bench_function("i3f29 sqrt 2", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I3F29::from_num(2)));
        })
    });
    g.bench_function("i3f29 sqrt bits::MAX/2+2", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I3F29::from_bits(i32::MAX / 2i32 + 2)));
        })
    });
    g.bench_function("i3f29 sqrt bits::MAX", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I3F29::from_bits(i32::MAX)));
        })
    });
    g.bench_function("i32f32 sqrt 2", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I32F32::from_num(2)));
        })
    });
    g.bench_function("i32f32 sqrt bits::MAX/2+2", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I32F32::from_bits(i64::MAX / 2i64 + 2)));
        })
    });
    g.bench_function("i32f32 sqrt bits::MAX", |b| {
        b.iter(|| {
            let _s = sqrt(black_box(I32F32::from_bits(i64::MAX)));
        })
    });

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
