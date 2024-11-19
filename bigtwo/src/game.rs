//! Run the entire Game Loop.

// use serde::Serialize;

mod check_player_can_play_hand;
use check_player_can_play_hand::check_player_can_play_hand;

mod next_player_id;
use next_player_id::next_player_id;

use crate::card::THREE_OF_CLUBS;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::get_ai_input::*;
use crate::player::Player;

use std::collections::BTreeSet;
use std::str::FromStr;

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

    pub is_start_trick: bool,
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
            is_start_trick: true,
        };

        game
    }
}

impl Game {
    pub fn is_on(&self) -> bool {
        for player in &self.players {
            if player.cards.is_empty() {
                return false;
            }
        }
        return true;
    }

    pub fn step(&mut self, input: &str) {
        let maybe_hand = Hand::from_str(input);
        if let Err(e) = maybe_hand {
            println!("cannot parse: {:?}", e);
            return;
        }
        let hand = maybe_hand.unwrap();

        let player = &mut self.players[self.current_player_idx];
        if let Err(e) =
            check_player_can_play_hand(self.played_hands.last(), player, &hand, self.is_start_trick)
        {
            println!("cannot play: {:?}", e);
            return;
        }

        if hand == Hand::Pass {
            self.passed_player_idxs.insert(self.current_player_idx);
        } else {
            player.remove_hand_from_cards(&hand);
            self.played_hands.push(hand);
        };

        self.current_player_idx = next_player_id(
            self.current_player_idx,
            &self.passed_player_idxs,
            NUM_PLAYERS,
        );

        if self.passed_player_idxs.len() == NUM_PLAYERS - 1 {
            self.passed_player_idxs.clear();
            self.is_start_trick = true;
        } else {
            self.is_start_trick = false;
        }
    }

    pub fn get_npc_turn(&mut self) -> Hand {
        println!("Player {}s turn", self.current_player_idx + 1);
        let player: &Player = &self.players[self.current_player_idx];
        let npc_play = if let Some(last) = self.played_hands.last() {
            if self.is_start_trick {
                start_trick_with_lowest_single(&player.cards)
            } else {
                play_smallest_single_or_pass(last, &player.cards)
            }
        } else {
            play_three_of_clubs(&player.cards)
        };

        println!("Player {} played {}", self.current_player_idx + 1, npc_play);
        npc_play
    }
}

/// Shuffle and Deal the cards just like a regular human dealer.
/// All players will receive 13 Cards each.
fn shuffle_and_deal_cards(players: &mut [Player; NUM_PLAYERS], mut deck: Deck) {
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
fn find_player_with_three_of_clubs(players: &[Player; NUM_PLAYERS]) -> usize {
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
