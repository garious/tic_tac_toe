extern crate rand;

mod board;
mod game;
mod player;
mod presenter;
mod rules;
mod script;
mod user_input;
mod view;

use board::Board;
use board::CellState::{Cross, Nought};
use game::Game;
use game::GameState::InProgress;
use player::*;
use player::computer::Computer;
use player::human::Human;
use player::strategy::lazy::Lazy;
use script::Script::{HowToPlay, Welcome};
use user_input::UserInput;
use view::View;

fn main() {
    let stdio = std::io::stdin();
    let input = stdio.lock();
    let output = std::io::stdout();
    let mut user_input = UserInput::new(input);
    let mut view = View::new(output);

    view.clear();
    view.print(Welcome.to_str());
    view.print(HowToPlay.to_str());
    user_input.read_line();

    let board = Board::new(3);
    let player_one = Human::new(Cross, user_input);
    let player_two = Computer::new(Nought, Lazy::new());

    let mut game = Game::new(board, player_one, player_two);

    while game.get_state() == &InProgress {
        game.play(&mut view);
    }
}
