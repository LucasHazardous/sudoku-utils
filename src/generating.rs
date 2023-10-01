use crate::solving;
use rand::{thread_rng, seq::SliceRandom};

pub fn generate_board(empty: u8) -> [[u8; 9]; 9] {
    let mut board = [[0; 9]; 9];
    fill_diagonal_squares(&mut board);

    solving::solve(&mut board);

    let mut zeros: Vec<usize> = (0..81).collect();
    zeros.shuffle(&mut thread_rng());
    for zero in zeros.into_iter().take(empty as usize) {
        board[zero / 9][zero % 9] = 0;
    }

    board
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