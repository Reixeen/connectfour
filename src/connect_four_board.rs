//use std::{char::CharTryFromError, io, process::exit};
//use strum::IntoEnumIterator;
//use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ConnectFourBoard {
    pub inner: u64,
}
impl ConnectFourBoard {
    pub fn new() -> Self {
        Self { inner: 0x0 }
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

    pub fn get_current_board(self, other_board: &ConnectFourBoard) -> ConnectFourBoard {
        ConnectFourBoard {
            // return a new board combining both boards - not the cleanest way of doing this.
            inner: self.inner | other_board.inner,
        }
    }

    pub fn place_token_in_spot(&mut self, slot: ConnectFourBoard) {
        self.inner |= slot.inner;
    }

    pub fn find_lowest_space_in_column(self, chosen_column: ConnectFourBoard) -> ConnectFourBoard {
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

pub struct sides {
    red: ConnectFourBoard,
    yellow: ConnectFourBoard,
}
