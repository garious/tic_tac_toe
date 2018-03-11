use board::Board;
use token::Token;
use token::Token::*;

pub fn is_game_over(board: &Board) -> bool {
    is_won(board) || is_draw(board)
}

pub fn get_winner(board: &Board) -> &Token {
    if win_for(&Cross, board) {
        return &Cross;
    } else if win_for(&Nought, board) {
        return &Nought;
    } else {
        return &Empty;
    }
}

fn is_won(board: &Board) -> bool {
    win_for(&Cross, board) || win_for(&Nought, board)
}

fn is_draw(board: &Board) -> bool {
    !is_won(board) && board.empty_cells().is_empty()
}

fn win_for(token: &Token, board: &Board) -> bool {
    board
        .partition()
        .iter()
        .any(|line| each_token_matches(line, token))
}

fn each_token_matches(line: &Vec<Token>, token: &Token) -> bool {
    line.iter().all(|cell| cell == token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::*;
    use token::Token::Empty;

    #[test]
    fn it_informs_if_game_not_over() {
        let board = Board::new(3);
        assert_eq!(false, is_game_over(&board));
    }

    #[test]
    fn it_informs_if_game_over_for_win() {
        let board = create_board_filling_cells(3, (0..9).collect());
        assert!(is_game_over(&board));
    }

    #[test]
    fn it_informs_if_game_is_over_for_draw() {
        let board = create_tied_board(3);
        assert!(is_draw(&board));
    }

    #[test]
    fn it_informs_if_game_is_won() {
        let mut board = Board::new(3);
        assert_eq!(false, is_won(&board));
        board = create_board_filling_cells(3, (0..9).collect());
        assert!(is_won(&board));
    }

    #[test]
    fn it_specifies_winner() {
        let board = create_board_filling_cells(3, (0..9).collect());
        assert_eq!(&Cross, get_winner(&board));
    }

    #[test]
    fn it_specifies_empty_as_draw_winner() {
        let board = create_tied_board(3);
        assert_eq!(&Empty, get_winner(&board));
    }

    #[test]
    fn it_informs_if_tokens_match() {
        let mixed_row = vec![Cross, Cross, Empty];
        let matching_row = vec![Cross, Cross, Cross];
        assert_eq!(false, each_token_matches(&mixed_row, &Cross));
        assert!(each_token_matches(&matching_row, &Cross));
    }

    #[test]
    fn it_informs_if_game_is_not_draw() {
        let mut board = Board::new(3);
        assert_eq!(false, is_draw(&board));
        board = create_board_filling_cells(3, (0..9).collect());
        assert_eq!(false, is_draw(&board));
    }

    #[test]
    fn it_informs_if_game_is_draw() {
        let board = create_tied_board(3);
        assert!(is_draw(&board));
    }
}
