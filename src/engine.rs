use crate::game::*;

/// A set finder.
/// Any struct that implements this trait can be benchmarked
pub trait SetFinder {
    /// Given a hand, find a set in the hand. Can return any found set.
    /// Should only return None if there are no sets in the hand.
    fn find(&self, hand: &[Card]) -> Option<Index>;
}

/// Run a user-supplied finder on a random game
pub struct Solver<F: SetFinder> {
    game: Game,
    finder: F,
}

impl<F: SetFinder> Solver<F> {
    pub fn new(finder: F) -> Solver<F> {
        Solver {
            game: Game::new(true),
            finder,
        }
    }

    /// Play a full game using the user's finder and return all found sets.
    /// Returns `None` if the given `SetFinder` is incorrect
    pub fn run(&mut self) -> Option<&[Set]> {
        while self.game.playable() {
            let hand = self.game.hand();

            let valid = self
                .finder
                .find(hand)
                .map(|set| self.game.add_set(set))
                .or_else(|| Some(self.game.draw_three()))
                .unwrap();

            if !valid {
                return None;
            }
        }

        Some(self.game.sets())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::oracle;

    #[test]
    fn test_oracle_engine() {
        let mut solver = Solver::new(oracle::Oracle);
        solver.run().unwrap().into_iter().for_each(|set| {
            assert!(set.is_valid());
        });
    }
}
