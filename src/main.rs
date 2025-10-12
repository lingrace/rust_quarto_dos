// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use std::fmt::Display;

const BOARD_SIZE: usize = 4;

struct Board {
    cells: [[i32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
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
    let board = Board::new();
    println!("{}", board);
}
