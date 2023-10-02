use crate::Sudoku;

impl Sudoku {
    pub fn validate(&self) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut columns = [[false; 9]; 9];
        let mut squares = [[false; 9]; 9];
    
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return false;
                }
                let key = (self.board[i][j]-1) as usize;
    
                if rows[i][key] || columns[j][key] || squares[i / 3 * 3 + j / 3][key] {
                    return false;
                }
    
                rows[i][key] = true;
                columns[j][key] = true;
                squares[i / 3 * 3 + j / 3][key] = true;
            }
        }
    
        true
    }
}