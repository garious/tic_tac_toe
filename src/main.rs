mod board;
mod io;
mod player;
mod presenter;

use board::*;
use io::*;
use player::*;

fn main() {
    let stdio = std::io::stdin();
    let input = stdio.lock();
    let output = std::io::stdout();
    let mut board = Board::new(3);
    let clear_sequence = format!("{}[2J\n", 27 as char);
    let mut io = IO::new(input, output, &clear_sequence);
    let player_one = Player::new(CellState::Cross);
    let player_two = Player::new(CellState::Nought);

    loop {
        io.clear();

        io.print(&presenter::view(&board));
        let selection = player_one.get_move(&board, &mut io);
        board.update(selection, *player_one.get_token());

        io.clear();

        io.print(&presenter::view(&board));
        let selection = player_two.get_move(&board, &mut io);
        board.update(selection, *player_two.get_token());
    }
}
