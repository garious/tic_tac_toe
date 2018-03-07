use std::io::{BufRead, Write};
use board::*;
use io::*;
use player::*;
use strategy::*;

#[derive(Debug, PartialEq)]
pub struct Computer<S> {
    token: CellState,
    strategy: S,
}

impl<S: Strategy> Computer<S> {
    pub fn new(token: CellState, strategy: S) -> Computer<S> {
        Computer { token, strategy }
    }
}

impl<S: Strategy> Player for Computer<S> {
    fn get_token(&self) -> &CellState {
        &self.token
    }

    fn get_move<R: BufRead, W: Write>(&self, board: &Board, _: &mut IO<R, W>) -> usize {
        self.strategy.decide(self.token, board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::CellState::*;
    use strategy::lazy::*;

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
        let player = Computer::new(Cross, Lazy::new());
        let mut board = Board::new(3);
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        update_cells(fill_spots, &mut board);
        let mut io = IO::new(&b"1"[..], Vec::new(), "");
        let selection = player.get_move(&board, &mut io);

        assert!(empty_spots.contains(&selection));
    }
}
