pub mod lazy;

use board::Board;

pub trait Strategy {
    fn decide(&self, board: &Board) -> usize;
}
