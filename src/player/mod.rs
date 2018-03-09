pub mod computer;
pub mod human;
pub mod strategy;

use board::Board;
use token::Token;

pub trait Player {
    fn get_token(&self) -> &Token;
    fn get_move(&mut self, board: &Board) -> Result<usize, String>;
}
