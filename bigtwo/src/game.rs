//! Run the entire Game Loop.

use crate::card::{Card, THREE_OF_CLUBS};
use crate::deck::Deck;
use crate::hand::try_from::*;
use crate::hand::Hand;
use crate::player::Player;

use std::collections::BTreeSet;

/// There are many variations of this game with non-4 numbers of players, but for now we focus on
/// the base game.
pub const NUM_PLAYERS: usize = 4;

/// Represents the current state of a Game
#[derive(Debug, Default)]
pub struct Game {
    /// history of all hands played by all players.
    /// the final played hand is the winner.
    /// anytime the same player plays twice in a raw, is because
    /// they won the Trick
    pub played_hands: Vec<(usize, Hand)>,

    /// players, and their cards
    pub players: [Player; NUM_PLAYERS],

    /// Used to index into a [Player; NUM_PLAYERS]
    pub current_player_idx: usize,

    /// Keeps track of all players who have passed so far this Trick
    pub passed_player_idxs: BTreeSet<usize>,
}

impl Game {
    pub fn get_npc_turn(&mut self) -> Hand {
        Hand::Pass
    }

    pub fn step(&mut self, hand: Hand) {
        self.played_hands.push((self.current_player_idx, hand));
        self.current_player_idx += 1;
        self.current_player_idx %= NUM_PLAYERS;
    }
}

/// Returned at the end of each Player's turn, informs the caller whether the Trick has ended (and
/// how), or ig the Trick continues
#[derive(Debug)]
enum StepStatus {
    /// Informs the caller that the previous attempted move failed.
    Retry,

    /// Informs the caller that this Trick is not over, returns the next player
    /// id.
    Continue,

    /// Informs the caller that this Trick ended without anybody winning the Game, so another Trick
    /// is needed.
    TrickOver(usize),

    /// Informs the caller that this Trick ended with somebody winning the Game.
    GameOver(usize),
}

/// Shuffle and Deal the cards just like a regular human dealer.
/// All players will receive 13 Cards each.
pub fn shuffle_and_deal_cards(players: &mut [Player; NUM_PLAYERS], mut deck: Deck) {
    println!("Dealing Cards...");
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    deck.cards[..].shuffle(&mut rng);

    let mut player_index: usize = 0;
    while let Some(card) = deck.cards.pop() {
        let index = player_index % NUM_PLAYERS;
        players[index].cards.push(card);
        player_index += 1;
    }
    for player in players {
        player.cards.sort();
    }
    assert_eq!(deck.cards.len(), 0);
}

///  Used to identify the player who was dealt the Three Of Clubs.
///  The game can only begin with the player that has the Three of Clubs.
pub fn find_player_with_three_of_clubs(players: &[Player; NUM_PLAYERS]) -> usize {
    for (index, player) in players.iter().enumerate() {
        if player.cards.contains(&THREE_OF_CLUBS) {
            return index;
        }
    }
    unreachable!();
}

/// Represents the possible ways that a string can fail to parse into a reasonable Hand.
#[derive(Debug)]
pub enum PlayerError {
    /// Not able to parse one of the Cards in this string.
    UnparseableInput(ParseHandError),

    /// Cards not found in players hand.
    NotPlayerCards,

    /// Attempting to play / pass out of turn
    NotPlayerTurn,
}

impl From<ParseHandError> for PlayerError {
    fn from(e: ParseHandError) -> Self {
        Self::UnparseableInput(e)
    }
}

pub fn check_play(player: &Player, input: &str) -> Result<Hand, PlayerError> {
    let mut cards = vec![];

    let card_strs: Vec<&str> = input.split_whitespace().collect();

    for card_str in card_strs {
        let maybe_card = card_str.to_uppercase().parse::<Card>();
        match maybe_card {
            Err(e) => {
                println!("error: could not understand {card_str}, {:?}", e);
                return Err(PlayerError::UnparseableInput(ParseHandError::BadCard(e)));
            }
            Ok(c) => cards.push(c),
        }
    }

    cards.sort();
    cards.reverse();

    if let Err(e) = Hand::sanitize_cards(&cards) {
        println!("error: sanitize cards failed {:?}", e);
        return Err(PlayerError::from(e));
    }

    match Hand::try_from_cards(&cards) {
        Ok(hand) => {
            if player.has_cards(&hand) {
                return Ok(hand);
            } else {
                return Err(PlayerError::NotPlayerCards);
            }
        }
        Err(e) => {
            println!("error: invalid hand {:?}", e);
            return Err(PlayerError::from(e));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::card::rank::*;
    use crate::card::suit::*;
    use crate::card::*;
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_get_user_input() {
        let input = "3C";
        let cards = vec_card_from_str("3C 3D 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        let hand = check_play(&player, input);
        assert!(matches!(hand, Ok(Hand::Lone(c)) if c == THREE_OF_CLUBS));

        const THREE_OF_DIAMONDS: Card = Card {
            rank: Rank::Three,
            suit: Suit::Diamonds,
        };
        const THREE_OF_SPADES: Card = Card {
            rank: Rank::Three,
            suit: Suit::Spades,
        };

        let mut input = "3C 3S 3D";
        let hand = check_play(&player, &input);
        assert!(
            matches!(hand, Ok(Hand::Trips(a, b, c)) if a == THREE_OF_SPADES && b == THREE_OF_DIAMONDS && c == THREE_OF_CLUBS,)
        );

        let mut input = "3G";
        let hand = check_play(&player, &input);
        assert!(matches!(
            hand,
            Err(PlayerError::UnparseableInput(ParseHandError::BadCard(
                ParseCardError::BadSuit(suit::ParseSuitError::BadChar(_))
            )))
        ));

        //     let expected_cards: [Card; 5] = [
        //         Card {
        //             rank: Rank::Seven,
        //             suit: Suit::Clubs,
        //         },
        //         Card {
        //             rank: Rank::Six,
        //             suit: Suit::Diamonds,
        //         },
        //         Card {
        //             rank: Rank::Five,
        //             suit: Suit::Hearts,
        //         },
        //         Card {
        //             rank: Rank::Four,
        //             suit: Suit::Diamonds,
        //         },
        //         THREE_OF_SPADES,
        //     ];
        //     let mut input = "3G\n3S 4D\n7C 6D 5H 4D 3S".as_bytes();
        //     let hand = check_play(&mut input);
        //     for (idx, card) in hand.cards().enumerate() {
        //         assert_eq!(*card, expected_cards[idx]);
        //     }

        //     let expected_cards: [Card; 5] = [
        //         Card {
        //             rank: Rank::Ten,
        //             suit: Suit::Diamonds,
        //         },
        //         Card {
        //             rank: Rank::Eight,
        //             suit: Suit::Diamonds,
        //         },
        //         Card {
        //             rank: Rank::Six,
        //             suit: Suit::Diamonds,
        //         },
        //         Card {
        //             rank: Rank::Four,
        //             suit: Suit::Diamonds,
        //         },
        //         Card {
        //             rank: Rank::Three,
        //             suit: Suit::Diamonds,
        //         },
        //     ];
        //     let mut input = "3G\n3S 4D\nTD 8D 6D 4D 3D".as_bytes();
        //     let hand = check_play(&mut input);
        //     for (idx, card) in hand.cards().enumerate() {
        //         assert_eq!(*card, expected_cards[idx]);
        //     }
    }
}
