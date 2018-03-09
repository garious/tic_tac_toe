use board::Board;
use player::Player;
use strategy::Strategy;
use token::Token;

#[derive(Debug, PartialEq)]
pub struct Computer<S> {
    token: Token,
    strategy: S,
}

impl<S: Strategy> Computer<S> {
    pub fn new(token: Token, strategy: S) -> Computer<S> {
        Computer { token, strategy }
    }
}

impl<S: Strategy> Player for Computer<S> {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_move(&mut self, board: &Board) -> Result<usize, String> {
        Ok(self.strategy.decide(board))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::{Cross, Nought};
    use strategy::lazy::Lazy;

    fn update_cells(indices: Vec<usize>, board: &mut Board) {
        for i in indices.iter() {
            match i {
                i if i % 2 == 0 => board.update(*i, Cross),
                _ => board.update(*i, Nought),
            }
        }
    }

    #[test]
    fn it_creates_new_player() {
        let player = Computer::new(Cross, Lazy::new());
        assert_eq!(Cross, player.token);
    }

    #[test]
    fn it_gets_player_token() {
        let player = Computer::new(Cross, Lazy::new());
        assert_eq!(&Cross, player.get_token());
    }

    #[test]
    fn it_gets_player_move() {
        let mut player = Computer::new(Cross, Lazy::new());
        let mut board = Board::new(3);
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        update_cells(fill_spots, &mut board);
        let selection = player.get_move(&board).unwrap();

        assert!(empty_spots.contains(&selection));
    }
}
