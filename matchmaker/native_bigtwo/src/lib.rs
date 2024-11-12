use bigtwo::deck::Deck;
use bigtwo::game::{find_player_with_three_of_clubs, shuffle_and_deal_cards};
use bigtwo::player::{make_player, Player};
use bigtwo::trick::{perform_trick, TrickResult, NUM_PLAYERS};

// #[rustler::nif]
// pub fn truly_random() -> i64 {
//     4 // Chosen by fair dice roll. Guaranteed to be random.
// }

struct Game {
    pub players: [Player; NUM_PLAYERS],
    pub is_first_trick: bool,
    pub current_player: usize,
}

static mut GAME: Game = Game {
    players: [make_player(), make_player(), make_player(), make_player()],
    is_first_trick: true,
    current_player: 0,
};

#[rustler::nif]
pub fn perform() {
    unsafe {
        shuffle_and_deal_cards(&mut GAME.players, Deck::new());

        GAME.current_player = find_player_with_three_of_clubs(&GAME.players);
        println!(
            "Player {0} has the Three of Clubs and may begin",
            GAME.current_player
        );

        let winner: usize = loop {
            let trick_result =
                perform_trick(GAME.current_player, &mut GAME.players, GAME.is_first_trick);
            GAME.is_first_trick = false;
            match trick_result {
                TrickResult::GameOver(winner) => break winner,
                TrickResult::NewTrick(new_starting_player_idx) => {
                    GAME.current_player = new_starting_player_idx;
                    println!("Player {0} wins the trick (everybody else passed) and starts the next trick", GAME.current_player);
                }
            }
        };
        println!("Game Over, Player {0} wins!!", winner);
    }
}

rustler::init!("native_bigtwo");
