use strum::IntoEnumIterator;

mod connect_four_board;
use crate::connect_four_board::AllBoards;
use crate::connect_four_board::Player;

fn print_intro_text() {
    println!("Welcome to Connect Four!")
}

fn main() {
    print_intro_text();

    let mut all_boards = AllBoards::new();

    all_boards.print_full_board();

    loop {
        for player in Player::iter() {
            all_boards.player_turn(player);
        }
    }
}
