pub mod lazy;
pub mod unbeatable;

use board::Board;

pub trait Strategy {
    fn decide(&self, board: &Board) -> usize;
}
