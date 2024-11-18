#![doc = include_str!("../../README.md")]
use bigtwo::game::{
    check_player_can_play_hand::{self, *},
    Game,
};

use bigtwo::player::Player;
// mod card;
// mod deck;
use bigtwo::hand::{self, Hand};
// mod player;
// mod trick;

// use bigtwo::game::perform_game;
use std::io;
use std::str::FromStr;

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");

    let mut game = Game::default();

    loop {
        let player: &mut Player = &mut game.players[game.current_player_idx];
        let hand = match game.current_player_idx {
            0 => get_player_turn(player, game.played_hands.last()),
            _ => game.get_npc_turn(),
        };
        game.step(hand);
    }
}

fn get_player_turn(player: &mut Player, last: Option<&Hand>) -> Hand {
    loop {
        let mut input = String::new();
        print!("=== > ");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the newline and print the input
                let trimmed_input = input.trim();
                println!("You entered: {}", trimmed_input);
                let maybe_hand = Hand::from_str(trimmed_input);
                match maybe_hand {
                    Ok(hand) => {
                        let maybe_ok = check_player_can_play_hand(last, player, &hand);
                        match maybe_ok {
                            Ok(_) => return hand,
                            Err(e) => {
                                print!("cannot play: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        print!("cannot parse: {:?}", e);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}
