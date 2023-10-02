mod solving;
mod generating;
mod validating;

#[derive(Clone)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9]
}