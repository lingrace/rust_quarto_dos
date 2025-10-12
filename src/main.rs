// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use std::fmt::Display;

const BOARD_SIZE: usize = 4;

type BoardCells = [[i32; BOARD_SIZE]; BOARD_SIZE];
struct Board {
    cells: BoardCells,
}

impl Board {
    fn new(cells: BoardCells) -> Self {
        Board { cells }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[-1; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_str = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            })
            .collect::<Vec<_>>()
            .join("\n");
        // TODO add line separator

        return write!(f, "{}", board_str);
    }
}
fn main() {
    println!("^_^");
    let board = Board::default();
    let interesting_board = Board::new([[0, 1, 2, 3], [4, 14, 6, 7], [0, 1, 2, 3], [4, 5, 6, 15]]);
    println!("{}", board);
    println!("{}", interesting_board);
}
