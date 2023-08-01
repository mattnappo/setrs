use crate::game::*;

/// A set finder
/// Any struct that implements this trait can be benchmarked
pub trait SetFinder {
    fn find(&self, hand: &[Card]) -> Option<Index>;
}

/// Run a user-supplied finder on a random game
struct Solver<F: SetFinder> {
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

    /// Play a full game using the user's finder
    /// Return all the found sets
    pub fn run(&mut self) -> Option<&[Set]> {
        // While the game is playable (TODO: whatever that means)
        // OR (thought): make find_set return None if no set

        // While game is playable (game is not playable when there are no more cards
        // in the deck and no sets in the hand, otherwise it is playable)
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
