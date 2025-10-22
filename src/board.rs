use std::{collections::HashSet, fmt::Display};

use crate::constants::{BOARD_SIZE, EMPTY_CELL_VALUE, NUM_PIECES};

pub type Piece = i8;
type BoardCells = [[Piece; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    InvalidPiece,
    PieceAlreadyUsed,
    CellAlreadyUsed,
}

#[derive(Debug)]
pub struct Board {
    cells: BoardCells,
    cell_width: usize, // for Display formatting purposes
}

impl Board {
    pub fn new(cells: BoardCells) -> Self {
        Board {
            cells,
            ..Default::default()
        }
    }

    pub fn is_winning_line(line: &[Piece; BOARD_SIZE]) -> bool {
        let filtered_line: Vec<&Piece> = line.iter().filter(|&&x| x != EMPTY_CELL_VALUE).collect();

        let num_non_empty_cells = filtered_line.len();
        let cumulative_bit_and = filtered_line.iter().fold(0b1111, |acc, x| acc & *x);
        let cumulative_bit_or = filtered_line.iter().fold(0b0000, |acc, x| acc | *x);

        // TODO: remove or turn into a --debug log
        // #[cfg(debug_assertions)]
        // {
        //     println!(
        //         "line: {:?}, num_pieces: {}, cumulative_bit_and: {}, cumulative_bit_or: {}",
        //         line, num_non_empty_cells, cumulative_bit_and, cumulative_bit_or
        //     );
        // }

        return num_non_empty_cells == BOARD_SIZE
            && (cumulative_bit_and != 0b0000 || cumulative_bit_or != 0b1111);
    }

    pub fn available_pieces(&self) -> Vec<Piece> {
        let mut all_available_pieces: HashSet<Piece> = (0..NUM_PIECES as i8).collect();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.cells[i][j] != EMPTY_CELL_VALUE {
                    all_available_pieces.remove(&self.cells[i][j]);
                }
            }
        }
        let mut all_available_pieces: Vec<Piece> = all_available_pieces.into_iter().collect();
        all_available_pieces.sort();
        all_available_pieces
    }

    pub fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), BoardError> {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err(BoardError::OutOfBounds);
        }

        if piece as usize >= NUM_PIECES || piece < 0 {
            return Err(BoardError::InvalidPiece);
        }

        if self
            .cells
            .into_iter()
            .any(|row| row.into_iter().any(|cell| cell == piece))
        {
            return Err(BoardError::PieceAlreadyUsed);
        }

        if self.cells[row][col] != EMPTY_CELL_VALUE {
            return Err(BoardError::CellAlreadyUsed);
        }

        self.cells[row][col] = piece;

        Ok(())
    }

    pub fn is_won(&self) -> bool {
        // Check rows
        for row in self.cells {
            if Board::is_winning_line(&row) {
                return true;
            }
        }

        // Check columns
        for j in 0..BOARD_SIZE {
            let mut col = [EMPTY_CELL_VALUE; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                col[i] = self.cells[i][j];
            }
            if Board::is_winning_line(&col) {
                return true;
            }
        }

        // Check diagonals
        let mut diag = [EMPTY_CELL_VALUE; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            diag[i] = self.cells[i][i];
        }
        if Board::is_winning_line(&diag) {
            return true;
        }

        let mut cross_diag = [EMPTY_CELL_VALUE; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            cross_diag[i] = self.cells[i][BOARD_SIZE - 1 - i];
        }
        if Board::is_winning_line(&cross_diag) {
            return true;
        }

        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[EMPTY_CELL_VALUE; BOARD_SIZE]; BOARD_SIZE],
            cell_width: 4,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = "─".repeat(self.cell_width * BOARD_SIZE + 3 * (BOARD_SIZE - 1) + 4);
        let board_str = self
            .cells
            .iter()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|x| {
                        if *x != EMPTY_CELL_VALUE {
                            format!("{:width$}", x, width = self.cell_width)
                        } else {
                            format!("{:width$}", "", width = self.cell_width)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" │ ");
                format!("│ {} │", row_str)
            })
            .collect::<Vec<_>>()
            .join(&format!("\n{}\n", sep));

        return write!(f, "{}\n{}\n{}", sep, board_str, sep);
    }
}
