use crate::Sudoku;
use rand::{thread_rng, seq::SliceRandom};

impl Sudoku {
    pub fn new(empty: u8) -> Self {
        let mut sudoku = Self {
            board: [[0; 9]; 9]
        };
        
        Sudoku::fill_diagonal_squares(&mut sudoku.board);
    
        sudoku.solve();

        let mut zeros: Vec<usize> = (0..81).collect();
        zeros.shuffle(&mut thread_rng());
        zeros.into_iter().take(empty as usize).for_each(|zero| sudoku.board[zero / 9][zero % 9] = 0);
    
        sudoku
    }
    
    fn fill_diagonal_squares(board: &mut [[u8; 9]; 9]) {
        let mut rng = thread_rng();
    
        for sq in 0..3 {
            let base = sq * 3;
            let mut nums: Vec<u8> = (1..10).collect();
            nums.shuffle(&mut rng);
    
            for i in 0..3 {
                for j in 0..3 {
                    board[i+base][j+base] = nums.pop().unwrap();
                }
            }
        }
    }
}
