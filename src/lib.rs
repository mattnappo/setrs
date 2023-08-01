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

/*
macro_rules! swap {
    ($num1:literal, $num2:literal) => {
        let tmp = self.$num1;
        self.$num1 = self.$num2;
        self.$num2 = tmp;
    };
}
*/

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Red,
    Green,
    Purple,
}

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Oval,
    Diamond,
    Squiggle,
}

#[derive(Debug, PartialEq, Clone)]
enum Shading {
    Solid,
    Striped,
    Outlined,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
struct Card {
    color: Color,
    shape: Shape,
    shading: Shading,
    number: Number,
}

enum Feature {
    Color,
    Shape,
    Shading,
    Number,
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
#[derive(Clone, Copy)]
struct Index(usize, usize, usize);

impl Index {
    fn is_valid(&self, limit: usize) -> bool {
        let bounds = self.0 < limit && self.1 < limit && self.2 < limit;
        let equals = self.0 != self.1 && self.1 != self.2 && self.0 != self.2;
        bounds && equals
    }

    /// Sort the Index in ascending order
    fn sort(&mut self) {
        if self.0 > self.2 {
            let tmp = self.0;
            self.0 = self.2;
            self.2 = tmp;
        }

        if self.0 > self.1 {
            let tmp = self.0;
            self.0 = self.1;
            self.1 = tmp;
        }

        if self.1 > self.2 {
            let tmp = self.1;
            self.1 = self.2;
            self.2 = tmp;
        }
    }
}

/// A set
#[derive(Debug, PartialEq)]
struct Set(Card, Card, Card);

/// Return true iff the given feature is the same across the set
fn same(set: &Set, feat: Feature) -> bool {
    match feat {
        Feature::Color => set.0.color == set.1.color && set.1.color == set.2.color,
        Feature::Shape => set.0.shape == set.1.shape && set.1.shape == set.2.shape,
        Feature::Shading => set.0.shading == set.1.shading && set.1.shading == set.2.shading,
        Feature::Number => set.0.number == set.1.number && set.1.number == set.2.number,
    }
}

/// Return true iff the given feature is different across the set
fn different(set: &Set, feat: Feature) -> bool {
    match feat {
        Feature::Color => {
            set.0.color != set.1.color && set.1.color != set.2.color && set.0.color != set.2.color
        }
        Feature::Shape => {
            set.0.shape != set.1.shape && set.1.shape != set.2.shape && set.0.shape != set.2.shape
        }
        Feature::Shading => {
            set.0.shading != set.1.shading
                && set.1.shading != set.2.shading
                && set.0.shading != set.2.shading
        }
        Feature::Number => {
            set.0.number != set.1.number
                && set.1.number != set.2.number
                && set.0.number != set.2.number
        }
    }
}

impl Set {
    fn is_valid(&self) -> bool {
        let color = same(self, Feature::Color) || different(self, Feature::Color);
        let shape = same(self, Feature::Shape) || different(self, Feature::Shape);
        let shading = same(self, Feature::Shading) || different(self, Feature::Shading);
        let number = same(self, Feature::Number) || different(self, Feature::Number);
        color && shape && shading && number
    }
}

/// A set finder
/// Any struct that implements this trait can be benchmarked
trait Finder {
    fn find(&mut self, hand: &[Card]) -> Option<Index>;
}

/// Run a user-supplied finder on a random game
struct Solver<F: Finder> {
    game: Game,
    finder: F,
}

impl<F: Finder> Solver<F> {
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
        while self.game.playable {
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

        Some(&self.game.sets)
    }
}

/// The core state of a game of Set
struct Game {
    /// The cards left in the deck/hand
    deck: Vec<Card>,
    /// The number of cards in the current hand
    hand: usize,
    /// The found sets so far. sets+deck = full 81 cards
    sets: Vec<Set>,
    /// Number of sets on the board
    playable: bool,
}

impl Game {
    pub fn new(shuffle: bool) -> Game {
        let mut game = Game {
            deck: {
                let mut d = iproduct!(0..3, 0..3, 0..3, 0..3)
                    .map(Card::from)
                    .collect::<Vec<Card>>();
                if shuffle {
                    d.shuffle(&mut thread_rng());
                }
                d
            },
            hand: 12,
            sets: Vec::new(),
            playable: false,
        };
        Game::update_playable(&mut game);
        game
    }

    /// Count number of sets on the board using the `Oracle`
    fn update_playable(&mut self) {
        // consult the oracle
    }

    /// Return a set given card indices
    fn index_to_set(&self, index: &Index) -> Option<Set> {
        // Validate index
        if !index.is_valid(self.hand) {
            return None;
        }

        Some(Set(
            self.deck[index.0].clone(),
            self.deck[index.1].clone(),
            self.deck[index.2].clone(),
        ))
    }

    /* -- Gameplay functions -- */

    // See the current hand
    pub fn hand(&self) -> &[Card] {
        &self.deck[..self.hand]
    }

    /// Add a set to the found_sets iff it is valid, and draw 3 cards
    /// if possible. Returns true iff set is valid
    pub fn add_set(&mut self, mut cards: Index) -> bool {
        // lookup cards given indices
        let set = self.index_to_set(&cards).filter(Set::is_valid);
        match set {
            Some(s) => {
                self.sets.push(s); // Count this set
                cards.sort(); // Remove cards from deck in descending order
                self.deck.remove(cards.2);
                self.deck.remove(cards.1);
                self.deck.remove(cards.0);

                // Dont replace cards if hand > 12
                if self.hand > 12 {
                    self.hand -= 3;
                }
                true
            }
            None => false,
        }
    }

    /// Draw three more cards from the hand
    /// Returns false if there are sets on the board
    pub fn draw_three(&mut self) -> bool {
        // TODO: check if no sets first
        self.hand += 3;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deckgen() {
        let game = Game::new(false);
        assert_eq!(game.deck.len(), 81);
    }

    #[test]
    fn test_validset() {
        assert!(!Set(
            Card {
                color: Color::Purple,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::One
            },
            Card {
                color: Color::Red,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::Two
            },
            Card {
                color: Color::Red,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::Three,
            }
        )
        .is_valid());
        assert!(Set(
            Card {
                color: Color::Red,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::One
            },
            Card {
                color: Color::Red,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::Two
            },
            Card {
                color: Color::Red,
                shape: Shape::Squiggle,
                shading: Shading::Striped,
                number: Number::Three,
            }
        )
        .is_valid());
    }

    #[test]
    fn test_addset() {
        let mut game = Game::new(false);
        assert_eq!(game.hand().len(), 12);

        // Try to add valid set
        assert!(game.add_set(Index(0, 1, 2)));
        assert!(game.add_set(Index(0, 1, 2)));

        // Try to add invalid set
        assert!(!game.add_set(Index(0, 0, 2)));
        assert!(!game.add_set(Index(0, 1, 3)));

        assert_eq!(
            game.sets,
            vec![
                Set(
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Solid,
                        number: Number::One,
                    },
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Solid,
                        number: Number::Two,
                    },
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Solid,
                        number: Number::Three,
                    },
                ),
                Set(
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Striped,
                        number: Number::One,
                    },
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Striped,
                        number: Number::Two,
                    },
                    Card {
                        color: Color::Red,
                        shape: Shape::Oval,
                        shading: Shading::Striped,
                        number: Number::Three,
                    },
                ),
            ]
        )
    }
}
