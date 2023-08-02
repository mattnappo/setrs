pub mod engine;
pub mod game;
pub mod solvers;

#[macro_export]
macro_rules! create_benchmarks {
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
