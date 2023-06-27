use crate::{
    errors::ParseError,
    types::{Card, Hand, Rank},
};
use std::collections::{BTreeSet, HashMap, HashSet};

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

pub fn find_winners<'a>(hands_str: &[&'a str]) -> Result<Vec<&'a str>, ParseError> {
    let mut winners_str = Vec::new();
    let mut winner: Option<Hand> = None;
    for &hand_str in hands_str {
        let cards = Card::five_cards_from_str(hand_str)?;
        let hand = to_hand(cards);
        match winner {
            None => {
                winner = Some(hand);
                winners_str.push(hand_str);
            }
            Some(ref w) if hand > *w => {
                winner = Some(hand);
                winners_str.clear();
                winners_str.push(hand_str);
            }
            Some(ref w) if hand == *w => {
                winners_str.push(hand_str);
            }
            _ => {} // lesser hand, ignore
        }
    }
    Ok(winners_str)
}
