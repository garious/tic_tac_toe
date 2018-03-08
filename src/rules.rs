use board::{Board, CellState};
use board::CellState::*;

pub fn is_game_over(board: &Board) -> bool {
    is_won(board) || is_draw(board)
}

fn is_won(board: &Board) -> bool {
    win_for(&Cross, board) || win_for(&Nought, board)
}

fn is_draw(board: &Board) -> bool {
    !is_won(board) && board.empty_cells().is_empty()
}

fn win_for(token: &CellState, board: &Board) -> bool {
    board
        .partition()
        .iter()
        .any(|line| each_token_matches(line, token))
}

fn each_token_matches(line: &Vec<CellState>, token: &CellState) -> bool {
    line.iter().all(|cell| cell == token)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn update_cells(indices: Vec<usize>, board: &mut Board) {
        for i in indices.iter() {
            match i {
                i if i % 2 == 0 => board.update(*i, Cross),
                _ => board.update(*i, Nought),
            }
        }
    }

    fn draw(board: &mut Board) {
        let length = board.get_size().pow(2);
        let cross_spaces = [0, 2, 3, 7, 8];
        for i in 0..length {
            match cross_spaces.contains(&i) {
                true => board.update(i, Cross),
                false => board.update(i, Nought),
            }
        }
    }

    #[test]
    fn it_informs_if_game_is_over() {
        let mut board = Board::new(3);
        assert_eq!(false, is_game_over(&mut board));
        update_cells((0..9).collect(), &mut board);
        assert!(is_game_over(&mut board));
        draw(&mut board);
        assert!(is_draw(&mut board));
    }

    #[test]
    fn it_informs_if_game_is_won() {
        let mut board = Board::new(3);
        assert_eq!(false, is_won(&mut board));
        update_cells((0..9).collect(), &mut board);
        assert!(is_won(&mut board));
    }

    #[test]
    fn it_informs_if_tokens_match() {
        let mixed_row = vec![Cross, Cross, Empty];
        let matching_row = vec![Cross, Cross, Cross];
        assert_eq!(false, each_token_matches(&mixed_row, &Cross));
        assert!(each_token_matches(&matching_row, &Cross));
    }

    #[test]
    fn it_informs_if_game_is_a_draw() {
        let mut board = Board::new(3);
        assert_eq!(false, is_draw(&mut board));
        update_cells((0..9).collect(), &mut board);
        assert_eq!(false, is_draw(&mut board));
        draw(&mut board);
        assert!(is_draw(&mut board));
    }
}
