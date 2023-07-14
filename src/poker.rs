use crate::{
    errors::ParseError,
    types::{Card, Hand, Rank},
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter,
};

/// Module containing utility functions for converting Card array -> Hand, and finding the winning hands.
///
/// Given an array of cards, match it to a Hand.
///
/// # Example
/// ```rust
/// use poker::prelude::*;
/// let cards: [Card; 5] = [
///   "3S".parse().unwrap(),
///   "3H".parse().unwrap(),
///   "2S".parse().unwrap(),
///   "3D".parse().unwrap(),
///   "3C".parse().unwrap(),
/// ];
/// let hand = to_hand(cards);
/// assert_eq!(hand, Hand::FourOfAKind { quad: Rank::Three, remaining: Rank::Two });
/// ```
pub fn to_hand(cards: [Card; 5]) -> Hand {
    let mut ranks = Vec::with_capacity(5);
    let mut unique_suits = HashSet::new();
    let mut count_by_rank = HashMap::with_capacity(5);
    for card in cards.iter() {
        ranks.push(card.rank);
        unique_suits.insert(card.suit);
        *count_by_rank.entry(card.rank).or_insert(0_u8) += 1;
    }
    ranks.sort_unstable();
    let mut ranks_by_count: HashMap<u8, Vec<Rank>> = HashMap::with_capacity(4); // cannot get more than 4 of a kind
    for (rank, &count) in count_by_rank.iter() {
        ranks_by_count.entry(count).or_default().push(*rank);
    }

    let high = ranks[4];
    let low = ranks[0];
    let is_ace_leading_straight =
        ranks == vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];
    let is_straight =
        (low as u8 + 4 == high as u8 && count_by_rank.len() == 5) || is_ace_leading_straight;
    let high = if is_ace_leading_straight {
        ranks[3]
    } else {
        high
    }; // redefine high if ace leads a straight
    let is_flush = unique_suits.len() == 1;
    match (is_straight, is_flush) {
        (true, true) => Hand::StraightFlush { high },
        (true, false) => Hand::Straight { high },
        (false, true) => Hand::Flush {
            ranks: ranks.into_iter().collect::<BTreeSet<_>>(),
        },
        _ => {
            match (
                ranks_by_count.remove(&4),
                ranks_by_count.remove(&3),
                ranks_by_count.remove(&2),
                ranks_by_count.remove(&1),
            ) {
                (Some(quads), _, _, Some(singles)) => Hand::FourOfAKind {
                    quad: quads[0],
                    remaining: singles[0],
                },
                (_, Some(triples), Some(pairs), _) => Hand::FullHouse {
                    triple: triples[0],
                    pair: pairs[0],
                },
                (_, Some(triples), _, Some(singles)) => Hand::ThreeOfAKind {
                    triple: triples[0],
                    remaining: singles.into_iter().collect::<BTreeSet<Rank>>(),
                },
                (_, _, Some(pairs), Some(singles)) if pairs.len() == 2 => Hand::TwoPair {
                    low_pair: *pairs.iter().min().unwrap(), // safe unwrap(), we know there are 2 pairs
                    high_pair: *pairs.iter().max().unwrap(),
                    remaining: singles[0],
                },
                (_, _, Some(pairs), Some(singles)) => Hand::OnePair {
                    pair: pairs[0],
                    remaining: singles.into_iter().collect::<BTreeSet<Rank>>(),
                },
                (_, _, _, _) => Hand::HighCard {
                    ranks: ranks.into_iter().collect(),
                },
            }
        }
    }
}

/// Given an array of hand representations, convert them to Cards, then Hand, obtain the winner(s).
///
/// # Example
/// ```rust
/// use poker::prelude::*;
/// let hands = [
///   "3S 3H 2S 3D 4C",
///   "3S 3H 2S 3D 5C",
///   "3S 3H 2S 3D 6C",
///   "3S 3H 2S 3D 7C",
/// ];
///
/// let winners = find_winners(&hands).unwrap();
/// assert_eq!(winners, vec!["3S 3H 2S 3D 7C"]);
/// ```
pub fn find_winners<'a>(hands_str: &[&'a str]) -> Result<Vec<&'a str>, ParseError> {
    // get hands in iterative manner, allowing for early return on error
    let mut cards_and_hands = Vec::new();
    for &hand_str in hands_str {
        let cards = Card::five_cards_from_str(hand_str)?;
        let hand = to_hand(cards);
        cards_and_hands.push((hand_str, hand))
    }

    // sort by hand, descending
    cards_and_hands.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

    // find all winners
    let mut iter = cards_and_hands.into_iter();
    if let Some(first_winner) = iter.next() {
        let other_winners = iter
            .take_while(|(_, hand)| hand == &first_winner.1)
            .map(|(hand_str, _)| hand_str);
        Ok(iter::once(first_winner.0)
            .chain(other_winners)
            .collect::<Vec<_>>())
    } else {
        Ok(vec![])
    }
}
