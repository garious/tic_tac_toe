extern crate rand;

mod board;
mod game;
mod player;
mod rules;
mod script;
mod token;
mod ui;

use board::Board;
use game::Game;
use game::GameState::InProgress;
use player::*;
use player::computer::Computer;
use player::human::Human;
use player::strategy::unbeatable::Unbeatable;
use script::Script::{HowToPlay, Welcome};
use token::Token::{Cross, Nought};
use ui::input::{Input, UserInput};
use ui::view::View;

fn main() {
    let output = std::io::stdout();
    let mut user_input = UserInput::new();
    let mut view = View::new(output);

    view.clear();
    view.print(Welcome.to_str());
    view.print(HowToPlay.to_str());
    user_input.read_line();

    let board = Board::new(3);
    let player_one = Human::new(Cross, UserInput::new());
    let player_two = Computer::new(Nought, Unbeatable::new(Nought));

    let mut game = Game::new(board, Box::new(player_one), Box::new(player_two));

    while game.get_state() == &InProgress {
        game.play(&mut view);
    }

    game.reveal_winner(&mut view);
}
