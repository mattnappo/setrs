use crate::engine::SetFinder;
use crate::game::*;

pub struct SimpleSolver;

impl SetFinder for SimpleSolver {
    fn find(&self, hand: &[Card]) -> Option<Index> {
        // Implement your solver here!
        todo!()
    }
}
