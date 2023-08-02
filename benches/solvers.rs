use criterion::{black_box, criterion_group, criterion_main, Criterion};
use setrs::create_benchmarks;
use setrs::solvers;

create_benchmarks!({solvers::oracle::Oracle, "oracle"}); //, {solvers::simple::SimpleSolver, "simple_solver"});

criterion_main!(solvers);
