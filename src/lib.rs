use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::convert::From;

macro_rules! generate_from_impl {
    ($enum_name:ident { $($variant:ident),+ $(,)? }) => {
        impl From<i32> for $enum_name {
            fn from(t: i32) -> $enum_name {
                match t {
                    $(i if i == $enum_name::$variant as i32 => $enum_name::$variant,)+
                    _ => panic!("Invalid value for enum {}", stringify!($enum_name))
                }
            }
        }
    };
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Purple,
}

#[derive(Debug)]
enum Shape {
    Oval,
    Diamond,
    Squiggle,
}

#[derive(Debug)]
enum Shading {
    Solid,
    Striped,
    Outlined,
}

#[derive(Debug)]
enum Number {
    One,
    Two,
    Three,
}

generate_from_impl!(Color { Red, Green, Purple });
generate_from_impl!(Shape {
    Oval,
    Diamond,
    Squiggle
});
generate_from_impl!(Shading {
    Solid,
    Striped,
    Outlined
});
generate_from_impl!(Number { One, Two, Three });

#[derive(Debug)]
struct Card {
    color: Color,
    shape: Shape,
    shading: Shading,
    number: Number,
}

impl From<(i32, i32, i32, i32)> for Card {
    fn from(t: (i32, i32, i32, i32)) -> Card {
        Card {
            color: Color::from(t.0),
            shape: Shape::from(t.1),
            shading: Shading::from(t.2),
            number: Number::from(t.3),
        }
    }
}

/// An index in the hand
struct Index(usize, usize, usize);

/// A set
struct Set(Card, Card, Card);

impl Set {}

/// A set finder
/// Any struct that implements this trait can be benchmarked
trait Finder {
    fn find(&mut self, game: &Game) -> Index;
}

/// Run a user-supplied finder on a random game
struct Solver<F: Finder> {
    game: Game,
    finder: F,
}

impl<F: Finder> Solver<F> {
    pub fn new(finder: F) -> Solver<F> {
        Solver {
            game: Game::new(),
            finder,
        }
    }

    /// Play a full game using the user's finder
    /// Return all the found sets
    pub fn run(&mut self) -> Vec<Set> {}
}

/// The core state of a game of Set
struct Game {
    /// The cards left in the deck/hand
    deck: Vec<Card>,
    /// The number of cards in the current hand
    hand: usize,
    /// The found sets so far. sets+deck = full 81 cards
    sets: Vec<Set>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: {
                let mut d = iproduct!(0..3, 0..3, 0..3, 0..3)
                    .map(Card::from)
                    .collect::<Vec<Card>>();
                d.shuffle(&mut thread_rng());
                d
            },
            hand: 12,
            sets: Vec::new(),
        }
    }

    // See the current hand
    pub fn hand(&self) -> &[Card] {
        &self.deck[..self.hand]
    }

    fn get_card() {}

    /* -- Gameplay functions -- */

    /// Add a set to the found_sets iff it is valid, and draw 3 cards
    /// if possible. Returns true iff set is valid
    pub fn add_set(&mut self, cards: Index) -> bool {
        // lookup cards given indices
        // construct set object
        // set.validate()
        // remove cards from hand
        // add 3 more cards from deck
    }

    /// Draw three more cards from the hand
    /// Returns false if there are sets on the board
    pub fn draw_three() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let game = Game::new();
        assert_eq!(game.deck.len(), 81);
    }
}
