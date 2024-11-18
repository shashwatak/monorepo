use crate::card::{Card, THREE_OF_CLUBS};
use crate::hand::Hand;

pub fn play_three_of_clubs(cards: &Vec<Card>) -> Hand {
    assert_eq!(cards[0], THREE_OF_CLUBS);

    match cards[..] {
        [a, b, c, ..] => {
            if let Ok(trips) = Hand::try_trips(c, b, a) {
                trips
            } else if let Ok(pair) = Hand::try_pair(b, a) {
                pair
            } else {
                Hand::Lone(a)
            }
        } // _ => panic!("oop"),
    }
}

pub fn play_smallest_single_or_pass(hand: &Hand, cqrds: &Vec<Card>) -> Hand {
    if let Hand::Lone(c) = hand {
        for card in cards {
            if card > c {
                return Hand::Lone(*card);
            }
        }
    }
    Hand::Pass
}

pub fn start_trick_with_lowest_single(cards: &Vec<Card>) -> Hand {
    Hand::Lone(cards[0])
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::card::{rank::Rank, suit::Suit};
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_play_smallest_single_or_pass() {
        let hand_to_beat: Hand = "4H".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = play_smallest_single_or_pass(&hand_to_beat, &player_cards);
        assert!(matches!(
            hand,
            Hand::Lone(Card {
                rank: Rank::Four,
                suit: Suit::Spades
            })
        ));

        let hand_to_beat: Hand = "4H 4C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = play_smallest_single_or_pass(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));

        let hand_to_beat: Hand = "6C".parse().unwrap();
        let player_cards = vec_card_from_str("4D 4S 5C");
        let hand = play_smallest_single_or_pass(&hand_to_beat, &player_cards);
        assert!(matches!(hand, Hand::Pass));
    }

    #[test]
    fn test_play_three_of_clubs() {
        let cards = vec_card_from_str("3C 4C 5D 2S");
        let hand = play_three_of_clubs(&cards);
        assert!(matches!(hand, Hand::Lone(a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 5D 2S");
        let hand = play_three_of_clubs(&cards);
        assert!(matches!(hand, Hand::Pair(_, a) if a == THREE_OF_CLUBS));

        let cards = vec_card_from_str("3C 3D 3S 2S");
        let hand = play_three_of_clubs(&cards);
        assert!(matches!(hand, Hand::Trips(_, _, a) if a == THREE_OF_CLUBS));
    }
}
