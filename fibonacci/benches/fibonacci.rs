use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fibonacci::{binet, iterative, tail_recursive};

const BENCH_SIZE: u64 = 100;

fn bench_binet(c: &mut Criterion) {
    c.bench_function("binet", |b| b.iter(|| binet(black_box(BENCH_SIZE))));
}

fn bench_tail_recursive(c: &mut Criterion) {
    c.bench_function("tail recursive", |b| {
        b.iter(|| tail_recursive(black_box(BENCH_SIZE)))
    });
}

fn bench_iterative(c: &mut Criterion) {
    c.bench_function("non iterative", |b| {
        b.iter(|| iterative(black_box(BENCH_SIZE)))
    });
}

criterion_group!(benches, bench_binet, bench_tail_recursive, bench_iterative,);
criterion_main!(benches);
