use std::collections::BTreeSet;
use std::convert::TryInto;
use std::str::FromStr;

use crate::errors::ParseError;

// Enum to represent card ranks
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Rank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(ParseError::InvalidRank(s.to_string())),
        }
    }
}

// Enum to represent card suits
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn from_char(c: char) -> Result<Suit, ParseError> {
        match c {
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err(ParseError::InvalidSuit(c)),
        }
    }
}

// Struct to represent a single card
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let suit_char = chars.next_back().ok_or(ParseError::EmptyCardStr)?;
        let rank_str = chars.as_str();
        Ok(Card {
            rank: Rank::from_str(rank_str)?,
            suit: Suit::from_char(suit_char)?,
        })
    }
}

impl Card {
    pub fn five_cards_from_str(s: &str) -> Result<[Card; 5], ParseError> {
        let mut cards = Vec::with_capacity(5);
        for res in s.split_whitespace().map(Card::from_str) {
            match res {
                Result::Ok(card) => cards.push(card),
                Result::Err(e) => return Result::Err(e),
            }
        }

        let card_cnt = cards.len();
        cards
            .try_into()
            .map_err(|_| ParseError::InvalidCardCount(card_cnt))
    }
}

// Enum to represent a hand of cards
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {
    HighCard {
        ranks: BTreeSet<Rank>, // 5 long
    },
    OnePair {
        pair: Rank,
        remaining: BTreeSet<Rank>,
    },
    TwoPair {
        high_pair: Rank,
        low_pair: Rank,
        remaining: Rank,
    },
    ThreeOfAKind {
        triple: Rank,
        remaining: BTreeSet<Rank>, // 2 long,
    },
    Straight {
        high: Rank,
    },
    Flush {
        ranks: BTreeSet<Rank>, // 5 long,
    },
    FullHouse {
        triple: Rank,
        pair: Rank,
    },
    FourOfAKind {
        quad: Rank,
        remaining: Rank,
    },
    StraightFlush {
        high: Rank,
    },
}
