//! Represents a player in the game, could be AI or User.
pub mod get_ai_input;

use std::collections::BTreeSet;
use std::fmt::Display;

use crate::{card::Card, hand::Hand};

use serde::{Deserialize, Serialize};

/// Represents a player in the game.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Player {
    pub cards: Vec<Card>,
}

/// useful for printing
fn cards_to_string(cards: &[Card]) -> String {
    cards.iter().map(|card| format!("|{}|", card)).collect()
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", cards_to_string(&self.cards))
    }
}

impl Player {
    /// Used by the caller / game logic to take a Player's cards (ostensibly after the Player has
    /// played them legally).
    pub fn remove_hand_from_cards(&mut self, hand: &Hand) {
        assert!(self.has_cards(hand));
        for to_remove in hand.cards() {
            let index = self
                .cards
                .iter()
                .position(|card| *card == *to_remove)
                .unwrap();
            self.cards.remove(index);
        }
    }

    /// Used to make sure the Player actually has the cards they tried to play.
    pub fn has_cards(&self, hand: &Hand) -> bool {
        let cards: BTreeSet<&Card> = BTreeSet::from_iter(&self.cards);
        hand.cards().all(|card| cards.contains(card))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_has_cards() {
        let cards = vec_card_from_str("3C 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        let hand: Hand = "3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3S 3C".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "4S 4H 4D".parse().unwrap();
        assert!(player.has_cards(&hand));

        let hand: Hand = "3D".parse().unwrap();
        assert!(!player.has_cards(&hand));

        let hand: Hand = "4S 4H 4C".parse().unwrap();
        assert!(!player.has_cards(&hand));
    }

    #[test]
    fn test_remove_cards_from_hand() {
        let mut player = Player::default();
        player.cards = vec_card_from_str("3D 3S 5S 6S");
        player.remove_hand_from_cards(&"3S 3D".parse().unwrap());
        assert!(!player.cards.contains(&"3S".parse().unwrap()));
        assert!(!player.cards.contains(&"3D".parse().unwrap()));
        assert!(player.cards.contains(&"5S".parse().unwrap()));
        assert!(player.cards.contains(&"6S".parse().unwrap()));
    }
}
