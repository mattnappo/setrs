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
