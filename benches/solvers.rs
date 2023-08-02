use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use setrs::create_solver_benchmark;
use setrs::solvers::*;

macro_rules! create_benchmark {
    ($({ $solver:expr, $name:literal }),* $(,)?) => {
        paste::paste! {
            $(pub fn [<$name _benchmarker>](c: &mut Criterion) {
                c.bench_function(stringify!($name), |b| {
                    b.iter(|| {
                        let mut runner = setrs::engine::Solver::new($solver);
                        assert!(black_box(runner.run()).is_some());
                    })
                });
            })*
            criterion_group!(solvers, $([<$name _benchmarker>],)* );
        }
    };
}

create_benchmark!({oracle::Oracle, "oracle"}, {oracle::Oracle, "oracle2"});

criterion_main!(solvers);
