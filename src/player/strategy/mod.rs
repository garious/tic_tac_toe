pub mod lazy;

use board::*;

pub trait Strategy {
    fn decide(&self, token: CellState, board: &Board) -> usize;
}
