pub fn validate(board: &[[u8; 9]; 9]) -> bool {
    let mut rows = [[false; 9]; 9];
    let mut columns = [[false; 9]; 9];
    let mut squares = [[false; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                return false;
            }
            let key = (board[i][j]-1) as usize;

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

pub fn validate_at_position(board: &[[u8; 9]; 9], row: usize, column: usize) -> bool {
    let mut row_h = [false; 9];
    let mut column_h = [false; 9];
    let mut square_h = [false; 9];

    let mut square_row = row / 3 * 3;
    let mut square_column = column / 3 * 3;

    for i in 0..9 {
        if board[row][i] == 0 || row_h[board[row][i] as usize] ||
        board[i][column] != 0 || column_h[board[i][column] as usize] ||
        board[square_row][square_column] != 0 
        || square_h[board[square_row][square_column] as usize] {
            return false;
        }

        row_h[board[row][i] as usize] = true;
        column_h[board[i][column] as usize] = true;
        square_h[board[square_row][square_column] as usize] = true;

        square_column += 1;
        if square_column % 3 == 0 {
            square_column = column / 3 * 3;
            square_row += 1;
        }
    }

    true
}

