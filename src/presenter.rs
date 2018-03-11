use board::Board;

const NEW_LINE: &str = "\n";
const PLUS: &str = "+";
const SEGMENT: &str = "---";
const VBAR: &str = "|";

pub fn view(board: &Board) -> String {
    let mut board_display = String::new();

    for (i, cell) in board.get_cells().iter().enumerate() {
        let token = cell.to_str();
        let delimiter = match_cell_delimiter(i, board);
        board_display.push_str(&pad_sides(token));
        board_display.push_str(&delimiter);
    }

    board_display
}

fn match_cell_delimiter(index: usize, board: &Board) -> String {
    let size = board.get_row_size();
    let divider = generate_divider(board);
    let last_cell_index = (size * size) - 1;
    let row_end = size - 1;

    match index {
        index if index == last_cell_index => String::from(NEW_LINE),
        index if index % size == row_end => divider,
        _ => String::from(VBAR),
    }
}

fn generate_divider(board: &Board) -> String {
    let divider = vec![SEGMENT; board.get_row_size()];
    format!("\n{}\n", divider.join(PLUS))
}

fn pad_sides(token: &str) -> String {
    format!(" {} ", token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::{Cross, Nought};

    #[test]
    fn it_formats_size_3_board_to_string_view() {
        let divider = "\n---+---+---\n";
        let expected = vec![
            "   |   |   ",
            divider,
            "   |   | X ",
            divider,
            " O |   |   \n",
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
            "   |   |   |   \n",
        ].join("");
        let mut board = Board::new(4);
        board.update(5, Cross);
        board.update(8, Nought);

        assert_eq!(expected, view(&board));
    }
}
