use itertools::iproduct;
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

#[derive(Debug)]
struct Game {
    deck: Vec<Card>,
    position: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: iproduct!(0..3, 0..3, 0..3, 0..3).map(Card::from).collect(),
            position: 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        let game = Game::new();
        assert_eq!(game.deck.len(), 81);
    }
}
