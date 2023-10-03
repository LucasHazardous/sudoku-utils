use crate::Sudoku;

impl Sudoku {
    pub fn solve(&mut self) {
        let mut rows = [[false; 9]; 9];
        let mut columns = [[false; 9]; 9];
        let mut squares = [[false; 9]; 9];
    
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] != 0 {
                    let key = (self.board[i][j]-1) as usize;
                    rows[i][key] = true;
                    columns[j][key] = true;
                    
                    squares[i / 3 * 3 + j / 3][key] = true;
                }
            }
        }
    
        Sudoku::backtrack(&mut self.board, &mut rows, &mut columns, &mut squares);
    }
    
    fn find_next_empty(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {    
        board.iter().enumerate().find_map(|(i, row)| {
            row.iter().position(|&val| val == 0).map(|j| (i, j))
        })
    }
    
    fn backtrack(board: &mut [[u8; 9]; 9],
        rows: &mut [[bool; 9]; 9],
        columns: &mut [[bool; 9]; 9],
        squares: &mut [[bool; 9]; 9]) -> bool
    {
        if let Some((i, j)) = Sudoku::find_next_empty(board) {
            for num in 1..10 {
                if Sudoku::check(board, i, j, num, rows, columns, squares) {
                    if Sudoku::backtrack(board, rows, columns, squares) {
                        return true;
                    }
    
                    let key = (num-1) as usize;
        
                    rows[i][key] = false;
                    columns[j][key] = false;
                    squares[i / 3 * 3 + j / 3][key] = false;
                    board[i][j] = 0;
                }
            }
        
            false
        } else {
            true
        }
    }
    
    fn check(board: &mut [[u8; 9]; 9],
        i: usize, j: usize, val: u8,
        rows: &mut [[bool; 9]; 9],
        columns: &mut [[bool; 9]; 9],
        squares: &mut [[bool; 9]; 9]) -> bool
    {
        let key = (val-1) as usize;
        if !rows[i][key] && !columns[j][key] && !squares[i / 3 * 3 + j / 3][key] {
            rows[i][key] = true;
            columns[j][key] = true;
            squares[i / 3 * 3 + j / 3][key] = true;
            board[i][j] = val;
    
            true
        } else {
            false
        }
    }
}