//! Checks if a specified Player can actually play the Hand they are attempting to play.

use crate::card::THREE_OF_CLUBS;
use crate::hand::{order::order, Hand};
use crate::player::Player;

use std::fmt::Display;

/// Represents the different ways a Player's attempted Hand is not playable
#[derive(Debug)]
pub enum PlayHandError {
    /// The very first played hand of a game must have the Three Of Clubs
    NotThreeOfClubsToStartGame,

    /// The very first played hand of a trick cannot be a Pass
    PassedOnTrickStart,

    /// Attempted Hand must be the same number of cards as previous played Hand.
    NotMatching,

    /// Highest card of attempted Hand must be higher than the highest card of the previously
    /// played Hand.
    TooLow,

    /// Attempted Hand has cards not found in the player's cards.
    StolenCards,
}

impl Display for PlayHandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotThreeOfClubsToStartGame => write!(f, "must start game using Three of Clubs"),
            Self::PassedOnTrickStart => write!(f, "cannot pass at the start of a trick"),
            Self::NotMatching => write!(f, "wrong number of cards"),
            Self::TooLow => write!(f, "highest is not high enough"),
            Self::StolenCards => write!(f, "these cards are not in the players hand"),
        }
    }
}

/// Checks if a specified Player can actually play the Hand they are attempting to play.
/// Returns () if the Hand is playable, otherwise returns a specific PlayHandError.
pub fn check_player_can_play_hand(
    last_played_hand: Option<&Hand>,
    player: &Player,
    attempt: &Hand,
    is_start_trick: bool,
) -> Result<(), PlayHandError> {
    if let Hand::Pass = attempt {
        if !is_start_trick {
            Ok(())
        } else {
            Err(PlayHandError::PassedOnTrickStart)
        }
    } else if !player.has_cards(attempt) {
        // player may only play cards they possess
        Err(PlayHandError::StolenCards)
    } else if let Some(last) = last_played_hand {
        if is_start_trick {
            // can play anything to start a trick
            Ok(())
        } else {
            // use non-derived custom order to decide if Hand is playable on top of
            let ordering = order(last, attempt);
            match ordering {
                Some(std::cmp::Ordering::Greater) => Err(PlayHandError::TooLow),
                None => Err(PlayHandError::NotMatching),
                _ => Ok(()),
            }
        }
    } else {
        // must play three of clubs to start game
        let c = attempt.cards().last().unwrap();
        if c == &THREE_OF_CLUBS {
            Ok(())
        } else {
            Err(PlayHandError::NotThreeOfClubsToStartGame)
        }
    }
}

#[cfg(test)]
mod tests {

    // use std::{assert_matches, str::FromStr};

    use std::str::FromStr;

    use super::*;
    use crate::tests::test_util::vec_card_from_str;

    #[test]
    fn test_check_player_can_play_hand() {
        // start game w/ no hand to beat
        let player = Player {
            cards: vec_card_from_str("3C 3S"),
        };

        let hand: Hand = "3S".parse().unwrap();
        let res = check_player_can_play_hand(None, &player, &hand, true);
        assert!(matches!(
            res,
            Err(PlayHandError::NotThreeOfClubsToStartGame)
        ));

        let hand: Hand = "3C".parse().unwrap();
        let res = check_player_can_play_hand(None, &player, &hand, true);
        assert!(matches!(res, Ok(_)));

        // new trick begins with a Three of Clubs (ostensibly by player 0),
        let hand_to_beat = Hand::from_str("3C").unwrap();

        // player has a few cards
        let cards = vec_card_from_str("3D 3S 4H 4D 4S");
        let mut player = Player::default();
        player.cards = cards;

        // plays a Three of Spades
        let hand: Hand = "3S".parse().unwrap();
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Ok(())));

        // update hand
        let hand_to_beat = hand;

        // incorrectly plays a Three of Diamonds, reject
        let hand: Hand = "3D".parse().unwrap();
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Err(PlayHandError::TooLow)));

        // incorrectly plays a Pair of Fours, reject
        let hand: Hand = "4H 4D".parse().unwrap();
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Err(PlayHandError::NotMatching)));

        // incorrectly plays cards they don't have
        let hand: Hand = "2S".parse().unwrap();
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Err(PlayHandError::StolenCards)));

        // passes
        let hand: Hand = "".parse().unwrap();
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Ok(_)));

        // loses
        let hand_to_beat: Hand = "7D 6H 5C 4H 3D".parse().unwrap();
        let cards = vec_card_from_str("7C 6D 5H 4D 3S");
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Err(PlayHandError::TooLow)));

        // wins
        let cards = vec_card_from_str("7S 6D 5H 4D 3S");
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, false);
        assert!(matches!(res, Ok(_)));

        // start new trick, can play anything
        let cards = vec_card_from_str("3H");
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, true);
        assert!(matches!(res, Ok(_)));

        // start new trick, cannot pass
        let cards = vec![];
        let hand = Hand::try_from_cards(&cards[..]).unwrap();
        player.cards = cards;
        let res = check_player_can_play_hand(Some(&hand_to_beat), &player, &hand, true);
        assert!(matches!(res, Err(PlayHandError::PassedOnTrickStart)));
    }
}
