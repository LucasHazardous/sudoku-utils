use cursive::view::{Resizable, Nameable};
use cursive::{Cursive, CursiveExt};
use cursive::views::{EditView, Dialog, LinearLayout, NamedView, ResizedView, TextView};

use sudoku_utils::generating::generate_board;
use sudoku_utils::validating::validate;

type SudokuInput = NamedView<ResizedView<ResizedView<EditView>>>;

fn main() {
    let board = generate_board(2);

    let mut siv = Cursive::default();

    let _ = siv.load_theme_file("theme.toml");

    let mut grid_layout = LinearLayout::vertical();

    for i in 0..9 {
        let mut row_layout = LinearLayout::horizontal();

        for j in 0..9 {
            if board[i][j] == 0 {
                let edit_view = EditView::new()
                .max_content_width(1)
                .fixed_width(3)
                .fixed_height(1)
                .with_name(format!("cell_{}_{}", i, j));
                row_layout.add_child(edit_view);
            } else {
                let text_view = TextView::new(board[i][j].to_string())
                .fixed_width(3)
                .fixed_height(1);
                row_layout.add_child(text_view);
            }
        }
        grid_layout.add_child(row_layout);
    }

    let layout = LinearLayout::vertical()
        .child(grid_layout);

    siv.set_user_data(board);

    let dialog = Dialog::new()
        .title("Sudoku Game")
        .content(layout)
        .button("Validate", validate_board)
        .button("Quit", |s| s.quit());

    siv.add_layer(dialog);

    siv.run();
}

fn validate_board(s: &mut Cursive) {
    let mut sboard = s.user_data::<[[u8; 9]; 9]>().unwrap().clone();
    for i in 0..9 {
        for j in 0..9 {
            if sboard[i][j] != 0 {
                continue;
            }
            sboard[i][j] = s.call_on_name(&format!("cell_{}_{}", i, j), |view: &mut SudokuInput| {
                let content = view.get_mut().get_inner().get_inner().get_content().to_string();
                if content.len() > 0 {
                    content.parse().unwrap_or(0)
                } else {
                    0
                }
            }).unwrap();
        }
    }

    if validate(&sboard) {
        s.add_layer(Dialog::around(
            TextView::new("You won"))
            .button("Quit", |ss| ss.quit()
        ));
    }
}