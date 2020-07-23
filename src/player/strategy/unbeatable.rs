use board::Board;
use player::strategy::Strategy;
use rules;
use std::i32::{MAX, MIN};
use token::Token::{self, Cross, Nought};

#[derive(Debug, PartialEq)]
pub struct Unbeatable {
    max: Token,
    min: Token,
}

impl Unbeatable {
    pub fn new(token: Token) -> Unbeatable {
        Unbeatable {
            max: token,
            min: get_min_token(token),
        }
    }

    fn score(&self, depth: i32, board: &Board) -> i32 {
        match *rules::get_winner(board) {
            winner if winner == self.max => depth + 10,
            winner if winner == self.min => -depth - 10,
            _ => 0,
        }
    }

    fn current_token(&self, is_max: bool) -> Token {
        match is_max {
            true => self.max,
            false => self.min,
        }
    }

    fn get_best_option(
        &self,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        board: &Board,
        is_max: bool,
    ) -> (i32, usize) {
        let mut best_score;
        let mut best_move = 10;

        if rules::is_game_over(board) {
            best_score = self.score(depth, board);
            return (best_score, best_move);
        };

        for i in &board.empty_cells() {
            let token = self.current_token(is_max);
            let mock_board = board.clone().update(*i, token);
            best_score = self.get_best_option(depth - 1, alpha, beta, &mock_board, !is_max)
                .0;

            if is_max && alpha < best_score {
                alpha = best_score;
                best_move = *i;
            }

            if !is_max && beta > best_score {
                beta = best_score;
                best_move = *i;
            }

            if alpha >= beta {
                break;
            };
        }

        best_score = match is_max {
            true => alpha,
            false => beta,
        };

        (best_score, best_move)
    }
}

impl Strategy for Unbeatable {
    fn decide(&self, board: &Board) -> usize {
        let depth = board.empty_cells().len() as i32;
        self.get_best_option(depth, MIN, MAX, &board, true).1
    }
}

fn get_min_token(token: Token) -> Token {
    match token == Cross {
        true => Nought,
        false => Cross,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::*;
    use token::Token::Empty;

    #[test]
    fn it_creates_strategy_with_cross_max() {
        let unbeatable = Unbeatable::new(Cross);
        assert_eq!(Cross, unbeatable.max);
        assert_eq!(Nought, unbeatable.min);
    }

    #[test]
    fn it_creates_strategy_with_nought_max() {
        let unbeatable = Unbeatable::new(Nought);
        assert_eq!(Nought, unbeatable.max);
        assert_eq!(Cross, unbeatable.min);
    }

    #[test]
    fn it_returns_zero_as_score_for_draw() {
        let board = create_tied_board(3);
        let unbeatable = Unbeatable::new(Cross);
        assert_eq!(0, unbeatable.score(0, &board));
    }

    #[test]
    fn it_returns_score_for_max_win() {
        let max_win = vec![
            Cross, Cross, Cross, Nought, Empty, Nought, Empty, Empty, Empty
        ];
        let board = create_board_from_cells(max_win);
        let unbeatable = Unbeatable::new(Cross);
        assert_eq!(14, unbeatable.score(4, &board));
    }

    #[test]
    fn it_returns_score_for_min_win() {
        let max_win = vec![
            Cross, Empty, Cross, Nought, Nought, Nought, Cross, Empty, Empty
        ];
        let board = create_board_from_cells(max_win);
        let unbeatable = Unbeatable::new(Cross);
        assert_eq!(-13, unbeatable.score(3, &board));
    }

    #[test]
    fn it_returns_base_score_move_when_draw() {
        let board = create_tied_board(3);
        let unbeatable = Unbeatable::new(Cross);
        assert_eq!(
            (0, 10),
            unbeatable.get_best_option(0, MIN, MAX, &board, true)
        );
    }

    #[test]
    fn it_returns_score_move_for_game_over() {
        let win = vec![
            Cross, Empty, Nought, Empty, Cross, Nought, Empty, Empty, Cross
        ];
        let board = create_board_from_cells(win);
        let unbeatable = Unbeatable::new(Nought);
        assert_eq!(
            (-14, 10),
            unbeatable.get_best_option(4, MIN, MAX, &board, true)
        );
    }

    #[test]
    fn it_picks_last_remaining_spot() {
        let one_spot = vec![
            Cross, Nought, Cross, Cross, Nought, Cross, Nought, Empty, Nought
        ];
        let board = create_board_from_cells(one_spot);
        let unbeatable = Unbeatable::new(Nought);
        assert_eq!(
            (0, 7),
            unbeatable.get_best_option(1, MIN, MAX, &board, false)
        );
    }

    #[test]
    fn it_picks_corner_if_open() {
        let board = Board::new(3);
        let unbeatable = Unbeatable::new(Nought);
        assert_eq!(
            (0, 0),
            unbeatable.get_best_option(1, MIN, MAX, &board, true)
        );
    }

    #[test]
    fn it_blocks_min_player_win() {
        let block = vec![
            Nought, Cross, Empty, Empty, Cross, Empty, Empty, Empty, Empty
        ];
        let board = create_board_from_cells(block);
        let unbeatable = Unbeatable::new(Nought);
        assert_eq!(
            (0, 7),
            unbeatable.get_best_option(6, MIN, MAX, &board, true)
        );
    }
}
