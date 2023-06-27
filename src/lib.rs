mod errors;
mod poker;
use crate::poker::find_winners;
mod types;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
///
/// Given the potential for parsing errors, we first delegate to `find_winners` to do the heavy lifting, then log and swallow the error in `winning_hands`.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    match find_winners(hands) {
        Ok(winners) => Some(winners),
        Err(e) => {
            println!("failed to parse hands due to {}", e);
            None
        }
    }
}

/// Public API
pub mod prelude {
    pub use crate::errors::ParseError;
    pub use crate::poker::{find_winners, to_hand};
    pub use crate::types::{Card, Hand, Rank, Suit};
    pub use crate::winning_hands;
}
