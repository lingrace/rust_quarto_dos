// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use std::fmt::Display;

 
const BOARD_SIZE: usize = 4;


struct Board {
    cells: [[i32; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    fn new() -> Self {
        Board {
            cells:  [[-1; BOARD_SIZE]; BOARD_SIZE]
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str: String = "".to_string();
        for row in self.cells {
            if board_str != "" {
                board_str += "\n";
                // TODO: add line separator
            }
            let row_str_vec: Vec<String> = row.iter().map(|x| x.to_string()).collect(); 
            let row_str = row_str_vec.join(" | ");
            board_str += &row_str;
        }
        return write!(f, "{}", board_str);
    }
}
fn main() {
    println!("grrr");
    let board = Board::new();
    println!("{}", board);
}