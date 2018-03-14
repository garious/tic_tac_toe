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
use script::Script::{Goodbye, PlayAgain, Welcome};
use std::io::Write;
use ui::input::Input;
use ui::view::View;

const PLAY_AGAIN: &str = "y";

pub fn run<I: Input, W: Write>(mut user_input: I, mut view: &mut View<W>) -> Game {
    let mut game;

    welcome(&mut user_input, &mut view);

    loop {
        let mode_selection = setup::select_mode(&mut user_input, &mut view);
        game = setup::setup_game(mode_selection);

        while game.get_state() == &InProgress {
            game.next_turn(&mut view);
        }

        game.reveal_winner(&mut view);

        if select_exit(&mut user_input, &mut view) {
            break;
        };
    }

    goodbye(&mut view);
    game
}

fn welcome<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) {
    view.clear_print(Welcome.to_str());
    user_input.read_line();
}

fn select_exit<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) -> bool {
    view.print(PlayAgain.to_str());
    user_input.read_line().trim() != PLAY_AGAIN
}

fn goodbye<W: Write>(view: &mut View<W>) {
    view.clear_print(Goodbye.to_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::GameState::Over;
    use ui::input::tests::*;

    #[test]
    fn it_returns_a_completed_game() {
        let input = vec!["\n", "4", "n"];
        let mock_input = MockInput::new(input);
        let mut view = View::new(Vec::new());
        let game = run(mock_input, &mut view);

        assert_eq!(&Over, game.get_state());
    }
}
