// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use std::fmt::Display;

const BOARD_SIZE: usize = 4;

type BoardCells = [[i32; BOARD_SIZE]; BOARD_SIZE];
struct Board {
    cells: BoardCells,
    cell_width: usize,
}

impl Board {
    fn new(cells: BoardCells) -> Self {
        Board {
            cells,
            ..Default::default()
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[-1; BOARD_SIZE]; BOARD_SIZE],
            cell_width: 4,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = "─".repeat(self.cell_width * BOARD_SIZE + 3 * (BOARD_SIZE - 1) + 2);
        let board_str = self
            .cells
            .iter()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|x| format!("{:width$}", x, width = self.cell_width))
                    .collect::<Vec<_>>()
                    .join(" │ ");
                format!("│{}│", row_str)
            })
            .collect::<Vec<_>>()
            .join(&format!("\n{}\n", sep));
        // TODO add line separator

        return write!(f, "{}\n{}\n{}", sep, board_str, sep);
    }
}
fn main() {
    println!("-_-");
    let board = Board::default();
    let interesting_board = Board::new([[0, 1, 2, 3], [4, 14, 6, 7], [0, 1, 2, 3], [4, 5, 6, 15]]);
    println!("{}", board);
    println!("{}", interesting_board);
}
