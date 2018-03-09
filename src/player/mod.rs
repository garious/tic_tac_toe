pub mod computer;
pub mod human;
pub mod strategy;

use board::{Board, CellState};

pub trait Player {
    fn get_token(&self) -> &CellState;
    fn get_move(&mut self, board: &Board) -> Result<usize, String>;
}
