/// Module for error definitions.

/// Errors typical to parsing of Cards/Hands.
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("empty Card")]
    EmptyCardStr,
    #[error("invalid Suit {0}")]
    InvalidSuit(char),
    #[error("invalid Rank {0}")]
    InvalidRank(String),
    #[error("invalid Card count {0}")]
    InvalidCardCount(usize),
}
