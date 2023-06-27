mod errors;
mod poker;
use crate::poker::find_winners;
mod types;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    match find_winners(hands) {
        Ok(winners) => Some(winners),
        Err(e) => {
            println!("failed to parse hands due to {}", e);
            None
        }
    }
}
