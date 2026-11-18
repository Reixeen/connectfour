use std::{fs::OpenOptions, io, io::Write, process::exit};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod connect_four_board;
use crate::connect_four_board::ConnectFourBoard;
use crate::connect_four_board::WinningLocations;

#[derive(Debug, EnumIter, Display)]
pub enum Player {
    Red,
    Yellow,
}

fn print_intro_text() {
    println!("Welcome to Connect Four!")
}

fn print_full_board(red_board: &ConnectFourBoard, yellow_board: &ConnectFourBoard) {
    // adjusted the Rustic Chess board print function to work for:
    // 6-tall, 7-wide, boards + 2 players
    let red = red_board.inner;
    let yellow = yellow_board.inner;

    const LAST_BIT: u64 = 63; // start at final bit
    // bitboard unfurls from bottom left corner
    // we need to print it from the top left.

    for row in 2..8 {
        // skip the top 2 rows Connect Four does not use
        for column in (1..8).rev() {
            // ignore end column + print from top row
            let mask = 1u64 << (LAST_BIT - (row * 8) - column);
            match (red & mask, yellow & mask) {
                // each slot can only have 1 player's token - check both at the same time
                (0, 0) => print!("_ "), // empty slot for both boards
                (_, 0) => print!("R "), // only red board has a token there
                (0, _) => print!("Y "), // only yellow board has a token there
                (_, _) => panic!("Both boards have a token here??"), // not reachable due to token placement logic - required for exhasutive match
            }
        }
        println!(); // new line at the end of each row
    }
    println!("1 2 3 4 5 6 7\n") // column labelling 
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

fn player_turn(
    // function to collate all turn logic
    player: Player,
    mut red_board: ConnectFourBoard,
    mut yellow_board: ConnectFourBoard,
) -> (ConnectFourBoard, ConnectFourBoard) {
    println!("It is player {player}'s turn:");

    let desired_column = get_chosen_column();
    let current_board = red_board.get_current_board(&yellow_board);
    let lowest_spot = current_board.find_lowest_space_in_column(desired_column);

    match player {
        // not the most idiomatic solution...
        Player::Red => {
            red_board.place_token_in_spot(lowest_spot);
            match check_win(&red_board) {
                Ok(msg) => {
                    victory_exit(msg, &red_board, &yellow_board);
                }
                Err(msg) => println!("{msg}"),
            };
        }
        Player::Yellow => {
            yellow_board.place_token_in_spot(lowest_spot);
            match check_win(&yellow_board) {
                Ok(msg) => {
                    victory_exit(msg, &red_board, &yellow_board);
                }
                Err(msg) => println!("{msg}"),
            };
        }
    }

    print_full_board(&red_board, &yellow_board);

    (red_board, yellow_board)
}

fn check_win(player_board: &ConnectFourBoard) -> Result<&'static str, &'static str> {
    // only need to scan the current players board
    let player_board: u64 = player_board.inner; // extract u64 only once instead of per check
    for check_type in WinningLocations::WINNING_LOCATIONS {
        let starting_column = check_type.starting_column;
        let max_row = check_type.max_row;
        let ending_column = check_type.ending_column;
        let mask = check_type.win_type.inner; // get mask and get u64
        let row_offset = ConnectFourBoard::ROW_OFFSET; // << offset of 8 moves the check up 1 row in 8x8 bitboard

        // currently scans all possible win patterns and positions
        for column in starting_column..ending_column {
            for row in 0..max_row {
                // all checks start at row 0 upwards, but not all have the same height
                let offset = column + (row_offset * row);
                let mask = mask << offset;
                if mask == (mask & player_board) {
                    return Ok("Winner!");
                }
            }
        }
    }
    Err("Oh no")
}

fn victory_exit(msg: &str, red_board: &ConnectFourBoard, yellow_board: &ConnectFourBoard) {
    println!("{msg}");
    print_full_board(&red_board, &yellow_board);
    write_board_to_file(&red_board, &yellow_board);
    exit(1)
}

fn generate_completed_boards(num: i32) {
    // select a random win pattern
    // place it in a random valid location
    // fill in the full board with all the other tokens of a game, or use a completed template with no wins
    // save both player boards to a file
}

fn write_board_to_file(
    red_board: &ConnectFourBoard,
    yellow_board: &ConnectFourBoard,
) -> std::io::Result<()> {
    let red_board = red_board.inner;
    let yellow_board = yellow_board.inner;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("completed_games.txt")?;
    writeln!(&mut file, "{red_board},{yellow_board}")?;
    Ok(())
}

fn main() {
    print_intro_text();

    let mut red_board = ConnectFourBoard::new();
    let mut yellow_board = ConnectFourBoard::new();

    //let mut red_board = ConnectFourBoard { inner: 0x505051D };
    // partly filled board for testing

    //let mut yellow_board = ConnectFourBoard { inner: 0x400202222 };
    // partly filled board for testing

    print_full_board(&red_board, &yellow_board);
    loop {
        for player in Player::iter() {
            (red_board, yellow_board) = player_turn(player, red_board, yellow_board);
        }
    }
}
