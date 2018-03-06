mod board;
mod presenter;

use board::*;

fn main() {
    let mut board = Board::new(3);
    print!("{}", presenter::view(&board));
    board.update(2, CellState::Cross);
    board.update(1, CellState::Nought);
    print!("{}", presenter::view(&board));
}
