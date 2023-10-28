type GRID = [[u8; 9]; 9];

pub struct Sudoku {
    board: GRID
}

impl Sudoku {
    pub fn new(board: GRID) -> Self {
        Self {
            board
        }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        // find an empty cell in the board
        // return None if all cells are filled
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }

        None 
    }

    fn check(&self, cell_selected: (usize, usize), value: u8) -> bool {
        let (x, y) = cell_selected;

        for i in 0..9 {
            if self.board[i][y] == value {
                return false;
            }
        }

        let sec_row = x / 3;
        let sec_col = y / 3; 
        
        for i in (sec_row * 3)..(sec_row * 3 + 3) {
            for j in (sec_col * 3)..(sec_col * 3 + 3) {
                if self.board[i][j] == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        if let Some((y, x)) = empty_cell {
            for val in 1..10 {
                if self.check((y, x), val) {
                    self.board[y][x] = val;
                    if self.solve() {
                        return true;
                    }

                    self.board[y][x] = 0
                }
            } 
        } else {
            return true;
        }

        false
    }

    pub fn print_board(&self) {
        let print_3_by_1 = |arr: Vec<u8>, last: bool| {
            let str = arr
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            if last {
                println!("{str}");
            } else {
                print!("{str} | ");
            }
        };

        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("- - - - - - - - - - - - - -")
            }

            print_3_by_1(self.board[i][0..3].to_vec(), false);
            print_3_by_1(self.board[i][3..6].to_vec(), false);
            print_3_by_1(self.board[i][6..9].to_vec(), true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_correct() {
        let board: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0]
        ];

            let mut sudoku = Sudoku::new(board);
            sudoku.print_board();
    }
}
