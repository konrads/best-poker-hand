use rstest::rstest;

use poker::winning_hands;

/// rstest implementation of poker.rs test.
#[rstest]
#[case::test_single_hand_always_wins(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])]
#[case::test_highest_card_of_all_hands_wins(&["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"], &["3S 4S 5D 6H JH"])]
#[case::test_a_tie_has_multiple_winners(&["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH", "3H 4H 5C 6C JD"], &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"])]
#[case::test_high_card_can_be_low_card_in_an_otherwise_tie(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])]
#[case::test_one_pair_beats_high_card(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])]
#[case::test_highest_pair_wins(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])]
#[case::test_two_pairs_beats_one_pair(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])]
#[case::test_two_pair_highest_pair_wins(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])]
#[case::test_two_pair_lowest_pair_wins(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])]
#[case::test_two_pair_tie_goes_to_high_card(&["2S QS 2C QD JH", "JD QH JS 8D QH"], &["JD QH JS 8D QH"])]
#[case::test_three_of_a_kind_beats_two_pair(&["2S 2H 2C 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])]
#[case::test_two_sets_of_three_of_a_kind_tie_goes_to_highest_ranked_triplet(&["2S 2H 2C 8D JH", "4S 4H 4C 8S JH"], &["4S 4H 4C 8S JH"])]
#[case::test_two_straights_highest_card_wins(&["4S 5H 6C 8D 7H", "5S 6H 7C 8D 9H"], &["5S 6H 7C 8D 9H"])]
#[case::test_two_straights_lowest_card_wins(&["4S 5H 6C 8D 7H", "3S 4H 5C 6D 7H"], &["4S 5H 6C 8D 7H"])]
#[case::test_two_flushes_highest_card_wins(&["2H 4H 6H 8H JH", "3D 5D 6D 8D 7D"], &["3D 5D 6D 8D 7D"])]
#[case::test_two_flushes_lowest_card_wins(&["4H 5H 6H 8H 7H", "2D 3D 6D 7D 8D"], &["4H 5H 6H 8H 7H"])]
#[case::test_two_flushes_tie_goes_to_high_card(&["4H 5H 6H 8H 7H", "2H 3H 6H 8H 7H"], &["4H 5H 6H 8H 7H"])]
#[case::test_flush_beats_a_straight(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])]
#[case::test_two_straight_flushes_highest_ranked_card_wins(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])]
#[case::two_pair_ranks(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])]
#[case::two_pairs_second_pair_cascade(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])]
#[case::two_pairs_last_card_cascade(&["2S QS 2C QD KH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])]
#[case::test_three_of_a_kind_ranks(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])]
#[case::test_three_of_a_kind_cascade_ranks(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])]
#[case::test_straight_beats_three_of_a_kind(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])]
#[case::test_aces_can_end_a_straight_high(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])]
#[case::test_aces_can_end_a_straight_low(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])]
#[case::test_straight_cascade(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])]
#[case::test_straight_scoring(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])]
#[case::test_flush_cascade(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])]
#[case::test_full_house_beats_a_flush(&["3H 6H 7H 8H 5H", "5S 3S 4S 2S 6S"], &["5S 3S 4S 2S 6S"])]
#[case::test_full_house_ranks(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])]
#[case::test_full_house_cascade(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])]
#[case::test_four_of_a_kind_beats_a_full_house(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])]
#[case::test_four_of_a_kind_ranks(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])]
#[case::test_four_of_a_kind_cascade(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])]
#[case::test_straight_flush_beats_four_of_a_kind(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])]
#[case::test_straight_flush_ranks(&["4S 5H 5S 5D 5C", "2S 3S 4S 5S 6S"], &["2S 3S 4S 5S 6S"])]
fn test_winners(#[case] hands: &[&str], #[case] winners: &[&str]) {
    let res = winning_hands(hands);
    assert_eq!(res.unwrap(), winners.to_vec());
}
