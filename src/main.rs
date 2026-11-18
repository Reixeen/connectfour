use std::{fs::OpenOptions, io, io::Write, process::exit};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod connect_four_board;
use crate::connect_four_board::AllBoards;
use crate::connect_four_board::ConnectFourBoard;
use crate::connect_four_board::Player;
use crate::connect_four_board::WinningLocations;

fn print_intro_text() {
    println!("Welcome to Connect Four!")
}

fn match_column_to_column(column: u8) -> Result<ConnectFourBoard, &'static str> {
    match column {
        1 => Ok(ConnectFourBoard::COLUMN_A),
        2 => Ok(ConnectFourBoard::COLUMN_B),
        3 => Ok(ConnectFourBoard::COLUMN_C),
        4 => Ok(ConnectFourBoard::COLUMN_D),
        5 => Ok(ConnectFourBoard::COLUMN_E),
        6 => Ok(ConnectFourBoard::COLUMN_F),
        7 => Ok(ConnectFourBoard::COLUMN_G),
        _ => Err("Invalid column number"),
    }
}

fn get_valid_integer() -> u8 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn get_chosen_column() -> ConnectFourBoard {
    println!("Please choose a column to place a token");

    let column_integer = get_valid_integer(); // only returns valid u8 integers 

    let mapped_column = match_column_to_column(column_integer); // only 7 valid columns, not u8 columns
    match mapped_column {
        Ok(verified_column) => verified_column, // return on valid column
        Err(msg) => {
            // if valid integer is not between 1 and 7 inclusive
            println!("{msg}");
            get_chosen_column() // call the function again - more fun than the loop{} in get_valid_integer()
        }
    }
}

impl AllBoards {
    pub fn player_turn(&mut self, player: Player) {
        println!("It is player {player}'s turn:");

        let desired_column = get_chosen_column();
        // this function prevents player_turn from being defined in connect_four_board.rs - for now.

        let lowest_spot = self.find_lowest_space_in_column(desired_column);

        self.place_token_in_spot(lowest_spot, &player);
        // putting the token into the current players board

        let board = self.get_board(&player);
        // getting a copy of the current players board to pass to check_win
        // will integrate this into check_win() in a future edit

        match board.check_win() {
            Ok(msg) => {
                println!("Player {player} wins!!!");
                self.victory_exit(msg);
            }
            Err(msg) => println!("{msg}"),
        };

        self.print_full_board();
    }
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
