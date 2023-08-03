use crate::engine::SetFinder;
use crate::game::*;

pub struct SweepSolver;

impl SetFinder for SweepSolver {
    fn find(&self, hand: &[Card]) -> Option<Index> {
        todo!()
    }
}
