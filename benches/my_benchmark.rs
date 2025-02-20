use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracing::run;


fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("run sphere", |b| b.iter(|| run::run_sphere()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);