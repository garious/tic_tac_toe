pub mod computer;
pub mod human;
pub mod strategy;

use std::io::{BufRead, Write};
use board::*;
use io::*;

pub trait Player {
    fn get_token(&self) -> &CellState;
    fn get_move<R: BufRead, W: Write>(&self, board: &Board, io: &mut IO<R, W>) -> usize;
}
