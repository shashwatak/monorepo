#![doc = include_str!("../../README.md")]

use bigtwo::card::cards_to_string;
use bigtwo::game::Game;

use std::io;

fn main() {
    println!("-------------------");
    println!("Welcome to Big Two!");
    println!("Submit hands by typing the cards in e.g. \"3C 3D 3S\"");
    println!("-------------------");

    let mut game = Game::default();

    while game.is_on() {
        let input = match game.current_player_idx {
            0 => {
                println!(
                    "Your remaining cards: {}",
                    cards_to_string(&game.players[game.current_player_idx].cards)
                );
                get_player_turn()
            }
            _ => game.get_npc_turn().to_string(),
        };
        game.step(input.as_str());
    }
}

fn get_player_turn() -> String {
    loop {
        let mut input = String::new();
        print!("=== > ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the newline and print the input
                let trimmed_input = input.trim();
                println!("You entered: {}", trimmed_input);
                return trimmed_input.to_string();
            }
            Err(e) => {
                eprintln!("Error reading input {}", e);
            }
        }
    }
}
