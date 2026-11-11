use std::io;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/* pub struct Position {
    sides: [ConnectFourBoard; 2],
}
 */
#[derive(Debug, EnumIter, Display)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ConnectFourBoard {
    inner: u64,
}
impl ConnectFourBoard {
    fn new() -> Self {
        Self { inner: 0x0 }
    }

    pub const EMPTY: Self = Self { inner: 0 };
    pub const FULL: Self = Self {
        inner: 0x7F7F7F7F7F7F, // board is full when COLUMNs 1-7 and ROWs 1-6 are full
    };

    pub const ROW_1: Self = Self { inner: 0x7F };
    pub const ROW_2: Self = Self { inner: 0x7F00 };
    pub const ROW_3: Self = Self { inner: 0x7F0000 };
    pub const ROW_4: Self = Self { inner: 0x7F000000 };
    pub const ROW_5: Self = Self {
        inner: 0x7F00000000,
    };
    pub const ROW_6: Self = Self {
        inner: 0x7F0000000000,
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

    pub const ROWS: [Self; 6] = [
        Self::ROW_1,
        Self::ROW_2,
        Self::ROW_3,
        Self::ROW_4,
        Self::ROW_5,
        Self::ROW_6,
    ];

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

    fn get_current_board(self, other_board: &ConnectFourBoard) -> ConnectFourBoard {
        ConnectFourBoard {
            // return a new board combining both boards - not the cleanest way of doing this.
            inner: self.inner | other_board.inner,
        }
    }

    fn place_token_in_spot(&mut self, slot: ConnectFourBoard) {
        self.inner |= slot.inner;
    }

    fn find_lowest_space_in_column(self, chosen_column: ConnectFourBoard) -> ConnectFourBoard {
        // input the full board and the chosen column for the next token
        let current_board_u64 = self.inner;

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
}

fn print_full_board(red_board: &ConnectFourBoard, yellow_board: &ConnectFourBoard) {
    let red = red_board.inner;
    let yellow = yellow_board.inner;

    const LAST_BIT: u64 = 63; // start at final bit

    for row in 2..8 {
        for column in (1..8).rev() {
            let mask = 1u64 << (LAST_BIT - (row * 8) - column);
            match (red & mask, yellow & mask) {
                (0, 0) => print!("_ "), // empty slot for both boards
                (_, 0) => print!("R "), // only red board has a token there
                (0, _) => print!("Y "), // only yellow board has a token there
                (_, _) => panic!("Both boards have a token here??"), // not reachable due to token placement logic
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

fn print_intro_text() {
    println!("Welcome to Connect Four!")
}

fn get_valid_integer() -> u8 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        match trimmed.parse::<u8>() {
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
            get_chosen_column() // request a new input 
        }
    }
}

fn player_turn(player: Player, mut red_board: ConnectFourBoard, mut yellow_board: ConnectFourBoard) -> (ConnectFourBoard, ConnectFourBoard) {
    println!("It is player {player}'s turn:");
    let desired_column = get_chosen_column();
    let current_board = red_board.get_current_board(&yellow_board);
    let lowest_spot = current_board.find_lowest_space_in_column(desired_column);
    match player {
        Player::Red => red_board.place_token_in_spot(lowest_spot),
        Player::Yellow => yellow_board.place_token_in_spot(lowest_spot),
            }
    
    return (red_board, yellow_board);
}

fn main() {
    print_intro_text();

    let mut red_board = ConnectFourBoard::new();
    let mut yellow_board = ConnectFourBoard::new();

    //let mut red_board = ConnectFourBoard { inner: 0x505051D }; // partly filled board for testing
    //let mut yellow_board = ConnectFourBoard { inner: 0x400202222 }; // partly filled board for testing

    print_full_board(&red_board, &yellow_board);

    loop {
        for player in Player::iter() {
            (red_board, yellow_board) = player_turn(player, red_board, yellow_board);
            
            print_full_board(&red_board, &yellow_board);
        }
    }
}
