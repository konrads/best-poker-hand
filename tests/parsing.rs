use rstest::rstest;

use poker::prelude::{find_winners, Card, ParseError};

/// rstest implementation of poker.rs test.
#[rstest]
#[case::test_too_many_cards(&["3S 3H 2S 3D 3C 4H"], "invalid Card count 6")]
#[case::test_too_few_cards(&["3S 3H 2S"], "invalid Card count 3")]
#[case::test_invalid_suit(&["3S 3H 2S 3H 6X"], "invalid Suit X")]
#[case::test_invalid_rank(&["3S 3H 2S 3H 666H"], "invalid Rank 666")]
fn test_parsing_hand(#[case] hands: &[&str], #[case] exp_err: &str) {
    let err = find_winners(hands).expect_err("Unexpected parse success");
    assert_eq!(err.to_string(), exp_err);
}

#[test]
fn test_parse_card() {
    let err: ParseError = "".parse::<Card>().expect_err("Unexpected parse success");
    assert_eq!(err.to_string(), "empty Card");
}
