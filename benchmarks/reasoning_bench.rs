use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn complex_reasoning_step(n: u64) -> u64 {
    // Simulated heavy reasoning task
    (1..n).product()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("complex_reasoning", |b| b.iter(|| complex_reasoning_step(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
