# SET Strategy Benchmarking Framework
Compare the speed of different [SET](https://en.wikipedia.org/wiki/Set_(card_game)) solving strategies!

# Usage
Simply just implement the `SetFinder` trait on any struct, and easily benchmark your solver!

```rust
pub trait SetFinder {
    fn find(&self, hand: &[Card]) -> Option<Index>;
}
```

The `find` function takes a slice of `Card`s, and returns the indices (`struct Index(usize,usize,usize)`)
of any cards in the slice that form a SET, and `None` otherwise.

## Details
Make sure to include your solver in `src/solvers/`, and make sure it is visible to the crate.

For example, if you want to create a solver called `mysolver`, perform the following steps:
1. Create the file `src/solvers/mysolver.rs` and implement the `SetFinder` trait here
2. Add `pub mod mysolver;` to `src/solvers/mod.rs` to ensure visibility to the benchmarking suite

To benchmark it, simply add your `impl SetFinder` struct to the benchmarker. In `benches/solvers.rs`,
add your solver:
```rust
create_benchmarks!(
    {solvers::oracle::Oracle, "oracle"},
    {solvers::mysolver::Solver::new(), "my_solver"} // Your solver
);
```

Then, run with `cargo bench`.
