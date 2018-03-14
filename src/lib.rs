extern crate rand;

pub mod board;
pub mod game;
pub mod player;
pub mod rules;
pub mod script;
pub mod setup;
pub mod token;
pub mod ui;

use game::GameState::InProgress;
use game::Game;
use script::Script::*;
use std::io::Write;
use token::Token::Empty;
use ui::color::Color;
use ui::input::Input;
use ui::presenter;
use ui::view::View;

const PLAY_AGAIN: &str = "y";

pub fn run<I: Input, W: Write>(mut user_input: I, mut view: &mut View<W>, color: &Color) -> Game {
    let mut game;

    welcome(&mut user_input, &mut view);

    loop {
        let mode_selection = setup::select_mode(&mut user_input, &mut view);
        game = setup::setup_game(mode_selection);

        while game.get_state() == &InProgress {
            prompt_turn(&mut game, view, color);
            game.next_turn();
        }

        show_winner(&mut game, view, color);

        if select_exit(&mut user_input, &mut view) {
            break;
        };
    }

    goodbye(&mut view);
    game
}

fn welcome<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) {
    view.update_with(Welcome.to_str());
    user_input.read_line();
}

fn prompt_turn<W: Write>(game: &mut Game, view: &mut View<W>, color: &Color) {
    let board = game.get_board();
    let board_length = board.get_length();
    let token = game.current_player_token();

    view.update_with(&presenter::view(&board, color));
    view.append_with(&format!(
        "[Player {}] ~ {}{}.",
        token.to_str(),
        PickSpot.to_str(),
        board_length
    ));
}

fn show_winner<W: Write>(game: &mut Game, view: &mut View<W>, color: &Color) {
    let board = game.get_board();
    let winner = game.get_winner();

    view.update_with(&presenter::view(&board, color));

    match winner {
        &Empty => view.append_with(Draw.to_str()),
        _ => view.append_with(&format!("{}{}", winner.to_str(), Wins.to_str())),
    };
}

fn select_exit<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) -> bool {
    view.append_with(PlayAgain.to_str());
    user_input.read_line().trim() != PLAY_AGAIN
}

fn goodbye<W: Write>(view: &mut View<W>) {
    view.update_with(Goodbye.to_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::GameState::Over;
    use ui::color::Color::Dim;
    use ui::input::tests::*;

    #[test]
    fn it_returns_a_completed_game() {
        let input = vec!["\n", "4", "n"];
        let mock_input = MockInput::new(input);
        let mut view = View::new(Vec::new());
        let game = run(mock_input, &mut view, &Dim);

        assert_eq!(&Over, game.get_state());
    }
}
