//! Run the entire Game Loop.

// use serde::Serialize;

pub mod check_player_can_play_hand;

mod next_player_id;
use next_player_id::next_player_id;

use crate::card::THREE_OF_CLUBS;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::get_ai_input::*;
use crate::player::Player;

use std::collections::BTreeSet;

/// There are many variations of this game with non-4 numbers of players, but for now we focus on
/// the base game.
pub const NUM_PLAYERS: usize = 4;

/// Represents the current state of a Game
#[derive(Debug)]
pub struct Game {
    /// history of all hands played by all players.
    /// the final played hand is the winner.
    pub played_hands: Vec<Hand>,

    /// players, and their cards
    pub players: [Player; NUM_PLAYERS],

    /// Used to index into a [Player; NUM_PLAYERS]
    pub current_player_idx: usize,

    /// Keeps track of all players who have passed so far this Trick
    pub passed_player_idxs: BTreeSet<usize>,
}

impl Default for Game {
    fn default() -> Self {
        let deck: Deck = Deck::new();
        let mut players: [Player; NUM_PLAYERS] = <[Player; NUM_PLAYERS]>::default();
        shuffle_and_deal_cards(&mut players, deck);
        let starting_player = find_player_with_three_of_clubs(&players);
        let game = Game {
            played_hands: vec![],
            players,
            current_player_idx: starting_player,
            passed_player_idxs: BTreeSet::default(),
        };

        game
    }
}

impl Game {
    pub fn get_npc_turn(&mut self) -> Hand {
        let player: &Player = &self.players[self.current_player_idx];
        if let Some(hand) = self.played_hands.last() {
            if self.passed_player_idxs.len() == NUM_PLAYERS - 1 {
                start_trick_with_lowest_single(&player.cards)
            } else {
                play_smallest_single_or_pass(hand, &player.cards)
            }
        } else {
            play_three_of_clubs(&player.cards)
        }
    }

    pub fn step(&mut self, hand: Hand) {
        self.current_player_idx = next_player_id(
            self.current_player_idx,
            &self.passed_player_idxs,
            NUM_PLAYERS,
        );
    }
}

/// Returned at the end of each Player's turn, informs the caller whether the Trick has ended (and
/// how), or ig the Trick continues
// #[derive(Debug)]
// enum StepStatus {
//     /// Informs the caller that the previous attempted move failed.
//     Retry,

//     /// Informs the caller that this Trick is not over, returns the next player
//     /// id.
//     Continue,

//     /// Informs the caller that this Trick ended without anybody winning the Game, so another Trick
//     /// is needed.
//     TrickOver(usize),

//     /// Informs the caller that this Trick ended with somebody winning the Game.
//     GameOver(usize),
// }

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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_get_user_input() {
        // let input = "3C";
        let cards = vec_card_from_str("3C 3D 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;
    }
}
