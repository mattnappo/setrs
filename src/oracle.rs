use crate::engine::SetFinder;
use crate::game::*;
use itertools::Itertools;

pub struct Oracle;

impl SetFinder for Oracle {
    fn find(&self, hand: &[Card]) -> Option<Index> {
        self.find_all(hand).into_iter().take(1).next()
    }
}

impl Oracle {
    fn find_all(&self, hand: &[Card]) -> Vec<Index> {
        hand.into_iter()
            .enumerate()
            .combinations(3)
            .map(|combination| {
                let index = Index::from_vec(combination.iter().map(|(i, _)| *i).collect()).unwrap();
                let set =
                    Set::from_vec(combination.iter().map(|(_, cards)| *cards).collect()).unwrap();
                (index, set)
            })
            .filter(|(_, set)| set.is_valid())
            .map(|(index, _)| index)
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let o = Oracle;
        let mut g = Game::new(false);

        for _ in 0..5 {
            println!("{g}");
            let hand = g.hand();
            assert!(g.add_set(o.find(hand).unwrap()));
        }
    }
}
