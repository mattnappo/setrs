use crate::engine::SetFinder;
use crate::game::*;
use itertools::Itertools;

pub struct Oracle;

impl SetFinder for Oracle {
    fn find(&self, hand: &[Card]) -> Option<Index> {
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
            .take(1)
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let o = Oracle;
        let mut g = Game::new(false);
        let set = o.find(g.hand());
        println!("hand: {:?}", g.hand());
        println!("set: {:?}", set);
        g.add_set(set.unwrap());
    }
}
