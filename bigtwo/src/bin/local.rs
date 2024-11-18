#![doc = include_str!("../../README.md")]
use bigtwo::game::{check_player_can_play_hand::*, Game};

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
            0 => loop {
                let hand = get_player_turn(player);
                if let Err(e) = check_player_can_play_hand(game.played_hands.last(), player, &hand)
                {
                    print!("cannot play: {:?}", e);
                    continue;
                }
                return hand;
            },
            _ => game.get_npc_turn(),
        };
        game.step(hand);
    }
}

fn get_player_turn(player: &mut Player) -> Hand {
    loop {
        let mut input = String::new();
        print!("=== > ");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the newline and print the input
                let trimmed_input = input.trim();
                println!("You entered: {}", trimmed_input);
                if let Ok(hand) = Hand::from_str(trimmed_input) {}
                match check_play(player, trimmed_input) {
                    Ok(hand) => player.remove_hand_from_cards(&hand),
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}
