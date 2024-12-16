use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_tracing::run;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run sphere", |b| b.iter(|| run::run_sphere()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);