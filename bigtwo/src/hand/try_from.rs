//! implements FromString for Hand, as well as helper function try_from_cards for making
//! valid hands out of Vec<Card> and &[Card]

use std::collections::BTreeSet;
use std::str::FromStr;

use crate::card::Card;
use crate::card::ParseCardError;
use crate::hand::Hand;

use std::fmt::{Display, Formatter};

/// Represents the possible ways that a string can fail to parse into a reasonable Hand.
#[derive(Debug)]
pub enum ParseHandError {
    /// Not able to parse one of the Cards in this string.
    BadCard(ParseCardError),
    /// Playing with a single deck, only one of each card allowed.
    DuplicateCard,
    /// For programmer convenience, must provide Cards in descending order>
    NotSortedDescending,
    /// All cards were parsed, in the correct order, but the result is not a valid Hand.
    InvalidHand(InvalidHandError),
}

impl From<ParseCardError> for ParseHandError {
    fn from(e: ParseCardError) -> Self {
        Self::BadCard(e)
    }
}

impl Display for ParseHandError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::BadCard(_) => write!(f, "{:?}", self),
            Self::DuplicateCard => write!(f, "{:?}", self),
            Self::NotSortedDescending => write!(f, "{:?}", self),
            Self::InvalidHand(_) => write!(f, "{:?}", self),
        }
    }
}

/// Represents the ways valid Cards can fail to combine into a valid Hand.
#[derive(Debug)]
pub enum InvalidHandError {
    /// Two cards of different Rank
    UnmatchedPair,
    /// Three cards, at least one is a different Rank
    UnmatchedTrips,
    /// Incorrect number of cards (0, 4, 6+)
    WrongQuantity,
    /// Not a valid Five Card Hand
    NotAFiveCardHand,
}

impl From<InvalidHandError> for ParseHandError {
    fn from(e: InvalidHandError) -> Self {
        Self::InvalidHand(e)
    }
}

impl Hand {
    /// Given a slice of Cards, either return a Hand, or an Error
    pub fn try_from_cards(cards: &[Card]) -> Result<Hand, ParseHandError> {
        match cards {
            [] => Ok(Hand::Pass),
            [a] => Ok(Hand::Lone(*a)),
            [a, b] => Ok(Hand::try_pair(*a, *b)?),
            [a, b, c] => Ok(Hand::try_trips(*a, *b, *c)?),
            [a, b, c, d, e] => Ok(Hand::try_fiver(*a, *b, *c, *d, *e)?),
            _ => Err(ParseHandError::InvalidHand(InvalidHandError::WrongQuantity)),
        }
    }

    /// Given two cards, return a Pair or an Error
    pub fn try_pair(first: Card, second: Card) -> Result<Hand, InvalidHandError> {
        assert!(second < first);
        if Hand::check_pair(&first, &second) {
            Ok(Hand::Pair(first, second))
        } else {
            Err(InvalidHandError::UnmatchedPair)
        }
    }

    /// Returns true if first and second make a valid Pair
    fn check_pair(first: &Card, second: &Card) -> bool {
        first.rank == second.rank
    }

    /// Given three cards, return a Trip or an Error
    pub fn try_trips(first: Card, second: Card, third: Card) -> Result<Hand, InvalidHandError> {
        assert!(third < second);
        assert!(second < first);
        if Hand::check_trips(&first, &second, &third) {
            Ok(Hand::Trips(first, second, third))
        } else {
            Err(InvalidHandError::UnmatchedTrips)
        }
    }

    /// Returns true if first, second, and third make a valid Trip
    fn check_trips(first: &Card, second: &Card, third: &Card) -> bool {
        first.rank == second.rank && second.rank == third.rank
    }

    /// Given five cards, return either a valid Hand or an error
    pub fn try_fiver(
        first: Card,
        second: Card,
        third: Card,
        fourth: Card,
        fifth: Card,
    ) -> Result<Hand, InvalidHandError> {
        assert!(fifth < fourth);
        assert!(fourth < third);
        assert!(third < second);
        assert!(second < first);

        let is_straight = Hand::check_straight(&first, &second, &third, &fourth, &fifth);
        let is_flush = Hand::check_flush(&first, &second, &third, &fourth, &fifth);
        if is_straight && is_flush {
            Ok(Hand::StraightFlush(first, second, third, fourth, fifth))
        } else if Hand::check_four_plus_kick(&first, &second, &third, &fourth, &fifth) {
            Ok(Hand::FourPlusKick(first, second, third, fourth, fifth))
        } else if Hand::check_full_house(&first, &second, &third, &fourth, &fifth) {
            Ok(Hand::FullHouse(first, second, third, fourth, fifth))
        } else if is_flush {
            Ok(Hand::Flush(first, second, third, fourth, fifth))
        } else if is_straight {
            Ok(Hand::Straight(first, second, third, fourth, fifth))
        } else {
            Err(InvalidHandError::NotAFiveCardHand)
        }
    }

    /// Returns true if the first through fifth are consecutive descending Rank.
    fn check_straight(
        first: &Card,
        second: &Card,
        third: &Card,
        fourth: &Card,
        fifth: &Card,
    ) -> bool {
        first.rank as usize == second.rank as usize + 1
            && second.rank as usize == third.rank as usize + 1
            && third.rank as usize == fourth.rank as usize + 1
            && fourth.rank as usize == fifth.rank as usize + 1
    }

    /// Returns true if the all five cards have the same suit.
    fn check_flush(first: &Card, second: &Card, third: &Card, fourth: &Card, fifth: &Card) -> bool {
        first.suit == second.suit
            && second.suit == third.suit
            && third.suit == fourth.suit
            && fourth.suit == fifth.suit
    }

    /// Returns true if the five cards are composed of a Pair and Trip
    fn check_full_house(
        first: &Card,
        second: &Card,
        third: &Card,
        fourth: &Card,
        fifth: &Card,
    ) -> bool {
        (Hand::check_pair(first, second) && Hand::check_trips(third, fourth, fifth))
            || (Hand::check_trips(first, second, third) && Hand::check_pair(fourth, fifth))
    }

    /// Returns true if the four cards are composed of a Quad and a Kicker
    fn check_four_plus_kick(
        first: &Card,
        second: &Card,
        _: &Card,
        fourth: &Card,
        fifth: &Card,
    ) -> bool {
        first.rank == fourth.rank || second.rank == fifth.rank
    }
    /// Return an Error if this slice of Cards is incoherent.
    pub fn sanitize_cards(cards: &[Card]) -> Result<(), ParseHandError> {
        let mut unique_cards: BTreeSet<&Card> = BTreeSet::new();
        for card in cards {
            unique_cards.insert(card);
        }
        if unique_cards.len() < cards.len() {
            return Err(ParseHandError::DuplicateCard);
        }

        for (i, card) in cards.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if cards[i - 1] < *card {
                return Err(ParseHandError::NotSortedDescending);
            }
        }

        Ok(())
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(hand_str: &str) -> Result<Hand, Self::Err> {
        let mut hand_str: String = hand_str.trim().to_string();
        hand_str.retain(|c| !r#"(),".;:'|"#.contains(c));
        if hand_str.is_empty() {
            return Ok(Hand::Pass);
        }
        let maybe_cards = hand_str.split(' ').collect::<Vec<&str>>();
        let mut cards: Vec<Card> = vec![];
        for maybe_card in maybe_cards {
            cards.push(maybe_card.parse()?);
        }

        Self::sanitize_cards(&cards[..])?;

        Self::try_from_cards(&cards)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_hand_to_from_string() {
        let hand = "AJ".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadCard(_))));

        let hand = "7D 5C 4C 3".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::BadCard(_))));

        let hand = "7D 5C 4C 3C".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(InvalidHandError::WrongQuantity))
        ));

        let hand = "3C 4D".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::NotSortedDescending)));

        let hand = "7D 3C 4C 5C 3C".to_string().parse::<Hand>();
        assert!(matches!(hand, Err(ParseHandError::DuplicateCard)));

        let hand = "2D 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(InvalidHandError::UnmatchedPair))
        ));

        let hand = "2S 2H 3S".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(
                InvalidHandError::UnmatchedTrips
            ))
        ));

        let hand = "2S 2H 5S 4S 3D".to_string().parse::<Hand>();
        assert!(matches!(
            hand,
            Err(ParseHandError::InvalidHand(
                InvalidHandError::NotAFiveCardHand
            ))
        ));
    }

    #[test]
    fn test_good_hand_to_from_string() {
        let good_hands = [
            "",
            "2S",
            "3D 3C",
            "KS KH KC",
            "8S 7D 6S 5C 4C",
            "AD TD 5D 4D 3D",
            "2S 2D 7S 7D 7C",
            "7S 7D 7C 4H 4D",
            "2S 5S 5H 5D 5C",
            "AS AH AD AC TS",
        ];
        for expected_hand in good_hands {
            println!("AAAAAAA: {expected_hand}");
            let hand = expected_hand.to_string().parse::<Hand>();
            println!("debug hand {hand:?}");
            assert!(matches!(hand, Ok(_)));
            let result_hand = hand.unwrap();
            println!("result hand {result_hand:?}");
            let result_hand = result_hand.to_string();
            println!("result string {result_hand}");
            assert_eq!(expected_hand, result_hand);
        }
    }
}
