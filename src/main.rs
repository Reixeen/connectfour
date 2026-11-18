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

fn generate_completed_boards(num: i32) {
    // select a random win pattern
    // place it in a random valid location
    // fill in the full board with all the other tokens of a game, or use a completed template with no wins
    // save both player boards to a file
}

impl AllBoards {
    pub fn player_turn(&mut self, player: Player) {
        let mut red_board = self.red;
        let mut yellow_board = self.yellow;
        println!("It is player {player}'s turn:");

        let desired_column = get_chosen_column();

        let lowest_spot = self.find_lowest_space_in_column(desired_column);

        match player {
            // not the most idiomatic solution...
            Player::Red => {
                red_board.place_token_in_spot(lowest_spot);
                match red_board.check_win() {
                    Ok(msg) => {
                        self.red = red_board;
                        self.victory_exit(msg);
                    }
                    Err(msg) => println!("{msg}"),
                };
            }
            Player::Yellow => {
                yellow_board.place_token_in_spot(lowest_spot);
                match yellow_board.check_win() {
                    Ok(msg) => {
                        self.yellow = yellow_board;
                        self.victory_exit(msg);
                    }
                    Err(msg) => println!("{msg}"),
                };
            }
        }

        self.red = red_board;
        self.yellow = yellow_board;

        self.print_full_board();
    }
}

fn main() {
    print_intro_text();

    let mut red_board = ConnectFourBoard::new();
    let mut yellow_board = ConnectFourBoard::new();

    //let mut red_board = ConnectFourBoard { inner: 0x505051D };
    // partly filled board for testing

    //let mut yellow_board = ConnectFourBoard { inner: 0x400202222 };
    // partly filled board for testing

    let mut all_boards = AllBoards {
        red: red_board,
        yellow: yellow_board,
    };

    all_boards.print_full_board();

    loop {
        for player in Player::iter() {
            all_boards.player_turn(player);
        }
    }
}
