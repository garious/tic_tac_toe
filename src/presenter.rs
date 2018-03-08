use board::{Board, CellState};
use board::CellState::*;

pub fn view(board: &Board) -> String {
    let mut board_display = String::new();

    for (i, cell) in board.get_cells().iter().enumerate() {
        let token = match_cell_to_token(cell);
        let delimiter = match_cell_delimiter(i, board);
        board_display.push_str(&pad_sides(token));
        board_display.push_str(&delimiter);
    }

    board_display
}

fn match_cell_to_token(cell: &CellState) -> &str {
    match *cell {
        Empty => " ",
        Cross => "X",
        Nought => "O",
    }
}

fn match_cell_delimiter(index: usize, board: &Board) -> String {
    let size = board.get_size();
    let divider = generate_divider(board);
    let last_cell_index = (size * size) - 1;
    let row_end = size - 1;

    match index {
        index if index == last_cell_index => String::from(""),
        index if index % size == row_end => divider,
        _ => String::from("|"),
    }
}

fn generate_divider(board: &Board) -> String {
    let divider = vec!["---"; board.get_size()];
    format!("\n{}\n", divider.join("+"))
}

fn pad_sides(token: &str) -> String {
    format!(" {} ", token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_formats_size_3_board_to_string_view() {
        let divider = "\n---+---+---\n";
        let expected = vec![
            "   |   |   ",
            divider,
            "   |   | X ",
            divider,
            " O |   |   ",
        ].join("");
        let mut board = Board::new(3);
        board.update(5, Cross);
        board.update(6, Nought);

        assert_eq!(expected, view(&board));
    }

    #[test]
    fn it_formats_size_4_board_to_string_view() {
        let divider = "\n---+---+---+---\n";
        let expected = vec![
            "   |   |   |   ",
            divider,
            "   | X |   |   ",
            divider,
            " O |   |   |   ",
            divider,
            "   |   |   |   ",
        ].join("");
        let mut board = Board::new(4);
        board.update(5, Cross);
        board.update(8, Nought);

        assert_eq!(expected, view(&board));
    }
}
