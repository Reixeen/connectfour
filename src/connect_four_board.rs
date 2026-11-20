use std::{char::CharTryFromError, io, process::exit};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use std::{fs::OpenOptions, io::Write};

#[derive(Debug, EnumIter, Display)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ConnectFourBoard {
    pub inner: u64,
}
impl ConnectFourBoard {
    pub fn new() -> Self {
        Self { inner: 0x0 }
    }

    pub fn place_token_in_spot(&mut self, slot: ConnectFourBoard) {
        self.inner |= slot.inner;
    }

    pub fn check_win(self) -> Result<&'static str, &'static str> {
        // only need to scan the current players board
        let player_board: u64 = self.inner; // extract u64 only once instead of per check
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

    pub const EMPTY: Self = Self { inner: 0 };
    pub const FULL: Self = Self {
        inner: 0x7F7F7F7F7F7F, // board is full when COLUMNs 1-7 and ROWs 1-6 are full
    };

    pub const COLUMN_A: Self = Self {
        inner: 0x10101010101,
    };
    pub const COLUMN_B: Self = Self {
        inner: 0x20202020202,
    };
    pub const COLUMN_C: Self = Self {
        inner: 0x40404040404,
    };
    pub const COLUMN_D: Self = Self {
        inner: 0x80808080808,
    };
    pub const COLUMN_E: Self = Self {
        inner: 0x101010101010,
    };
    pub const COLUMN_F: Self = Self {
        inner: 0x202020202020,
    };
    pub const COLUMN_G: Self = Self {
        inner: 0x404040404040,
    };

    pub const VERTICAL: Self = Self { inner: 0x1010101 };
    pub const HORIZONTAL: Self = Self { inner: 0xF };
    pub const FORWARD_DIAGONAL: Self = Self { inner: 0x8040201 };
    pub const BACKWARD_DIAGONAL: Self = Self { inner: 0x1020408 };

    pub const COLUMNS: [Self; 7] = [
        Self::COLUMN_A,
        Self::COLUMN_B,
        Self::COLUMN_C,
        Self::COLUMN_D,
        Self::COLUMN_E,
        Self::COLUMN_F,
        Self::COLUMN_G,
    ];

    pub const WIN_DIRECTIONS: [Self; 4] = [
        Self::VERTICAL,
        Self::HORIZONTAL,
        Self::FORWARD_DIAGONAL,
        Self::BACKWARD_DIAGONAL,
    ];

    pub const ROW_OFFSET: u8 = 8;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct AllBoards {
    pub red: ConnectFourBoard,
    pub yellow: ConnectFourBoard,
}

impl AllBoards {
    pub fn new() -> AllBoards {
        AllBoards {
            red: ConnectFourBoard::new(),
            yellow: ConnectFourBoard::new(),
        }
    }

    pub fn place_token_in_spot(&mut self, slot: ConnectFourBoard, player: &Player) {
        match player {
            Player::Red => self.red.inner |= slot.inner,
            Player::Yellow => self.yellow.inner |= slot.inner,
        }
    }

    pub fn get_board(&mut self, player: &Player) -> ConnectFourBoard {
        match player {
            Player::Red => self.red,
            Player::Yellow => self.yellow,
        }
    }

    pub fn combine_boards(self) -> u64 {
        return self.red.inner | self.yellow.inner;
    }

    pub fn find_lowest_space_in_column(self, chosen_column: ConnectFourBoard) -> ConnectFourBoard {
        // input the full board and the chosen column for the next token
        let current_board_u64 = self.combine_boards();

        // invert full board -> apply the column mask -> find least significant bit
        let empty_spots_in_column = !current_board_u64 & chosen_column.inner;
        let lsb: u64 = empty_spots_in_column.trailing_zeros().into();

        // shift LSB from 1 to get the bit 1 higher than LSB
        let lowest_space: u64 = 1u64 << (lsb);

        // return a board with only the desired spot filled
        ConnectFourBoard {
            inner: lowest_space,
        }
    }

    pub fn print_full_board(self) {
        // adjusted the Rustic Chess board print function to work for:
        // 6-tall, 7-wide, boards + 2 players
        let red = self.red.inner;
        let yellow = self.yellow.inner;

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

    pub fn victory_exit(self, msg: &str) {
        println!("{msg}");
        self.print_full_board();
        self.write_board_to_file();
        exit(1)
    }

    pub fn write_board_to_file(self) -> std::io::Result<()> {
        let red_board = self.red.inner;
        let yellow_board = self.yellow.inner;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("completed_games.txt")?;
        writeln!(&mut file, "{red_board},{yellow_board}")?;
        Ok(())
    }

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
            Err(_) => println!(""), // do nothing, next turn proceeds
        };

        self.print_full_board();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct WinningLocations {
    pub max_row: u8,         // all win types start at row 0 but vary in height
    pub starting_column: u8, // backward_diagonal win type is 4-wide and covers (0,1,2,3) when left-most
    pub ending_column: u8, // forward diagonal win type is 4-wide and covers (3,4,5,6) when right-most
    pub win_type: ConnectFourBoard,
}

impl WinningLocations {
    pub const VERTICAL: Self = Self {
        max_row: 3,
        starting_column: 0,
        ending_column: 7,
        win_type: ConnectFourBoard::VERTICAL,
    };
    pub const HORIZONTAL: Self = Self {
        max_row: 6,
        starting_column: 0,
        ending_column: 4,
        win_type: ConnectFourBoard::HORIZONTAL,
    };
    pub const FORWARD_DIAGONAL: Self = Self {
        max_row: 3,
        starting_column: 0,
        ending_column: 4,
        win_type: ConnectFourBoard::FORWARD_DIAGONAL,
    };
    pub const BACKWARD_DIAGONAL: Self = Self {
        max_row: 3,
        starting_column: 3,
        ending_column: 7,
        win_type: ConnectFourBoard::BACKWARD_DIAGONAL,
    };

    pub const WINNING_LOCATIONS: [Self; 4] = [
        Self::VERTICAL,
        Self::HORIZONTAL,
        Self::FORWARD_DIAGONAL,
        Self::BACKWARD_DIAGONAL,
    ];
}

pub fn match_column_to_column(column: u8) -> Result<ConnectFourBoard, &'static str> {
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

pub fn get_valid_integer() -> u8 {
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

pub fn get_chosen_column() -> ConnectFourBoard {
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
