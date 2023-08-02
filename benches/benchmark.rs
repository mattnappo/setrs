use criterion::{black_box, criterion_group, criterion_main, Criterion};
use setrs::engine;
use setrs::solvers::*;

pub fn oracle_benchmark(c: &mut Criterion) {
    c.bench_function("oracle", |b| {
        b.iter(|| {
            let mut runner = engine::Solver::new(oracle::Oracle);
            assert!(black_box(runner.run()).is_some());
        })
    });
}
criterion_group!(solvers, oracle_benchmark);

criterion_main!(solvers);
