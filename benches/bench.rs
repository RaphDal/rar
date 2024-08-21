use criterion::{criterion_group, criterion_main, Criterion};

const NUM_ITERS: usize = 10_000;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Benchmark");

    let scheduler = rar::builder::Builder::new().build();

    group.bench_function("rar", |b| {
        b.iter(|| {
            for _ in 0..NUM_ITERS {
                scheduler.block_on(async {});
            }
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
