mod board;
mod io;
mod presenter;

use board::*;
use io::*;

fn main() {
    let stdio = std::io::stdin();
    let input = stdio.lock();
    let output = std::io::stdout();
    let board = Board::new(3);
    let clear_sequence = format!("{}[2J\n", 27 as char);
    let mut console = IO::new(input, output, &clear_sequence);
    console.print(&presenter::view(&board));
}
