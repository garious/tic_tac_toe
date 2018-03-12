use board::Board;
use ui::color::Color;
use token::Token;
use token::Token::Empty;

const NEW_LINE: &str = "\n";
const PLUS: &str = "+";
const SEGMENT: &str = "---";
const VBAR: &str = "|";
const OFFSET: usize = 1;

pub fn view(board: &Board, color: &Color) -> String {
    let mut board_display = String::new();

    for (i, cell) in board.get_cells().iter().enumerate() {
        let token = determine_token(i, cell, color);
        let delimiter = match_cell_delimiter(i, board);
        board_display.push_str(&pad_sides(&token));
        board_display.push_str(&delimiter);
    }

    board_display
}

fn determine_token(index: usize, cell: &Token, color: &Color) -> String {
    match cell {
        &Empty => color.fill(&format!("{}", index + OFFSET)),
        _ => String::from(cell.to_str()),
    }
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
    use board::tests::*;
    use ui::color::Color::Normal;

    #[test]
    fn it_formats_size_3_board_to_string_view() {
        let divider = "\n---+---+---\n";
        let expected = vec![
            " 1 | 2 | 3 ",
            divider,
            " 4 | 5 | O ",
            divider,
            " X | 8 | 9 \n",
        ].join("");
        let board = create_board_filling_cells(3, vec![5, 6]);

        assert_eq!(expected, view(&board, &Normal));
    }

    #[test]
    fn it_formats_size_4_board_to_string_view() {
        let divider = "\n---+---+---+---\n";
        let expected = vec![
            " 1 | 2 | 3 | 4 ",
            divider,
            " 5 | O | 7 | 8 ",
            divider,
            " X | 10 | 11 | 12 ",
            divider,
            " 13 | 14 | 15 | 16 \n",
        ].join("");
        let board = create_board_filling_cells(4, vec![5, 8]);

        assert_eq!(expected, view(&board, &Normal));
    }
}
