extern crate rand;

pub mod board;
pub mod game;
pub mod player;
pub mod rules;
pub mod setup;
pub mod token;
pub mod ui;

use game::Game;
use game::GameState::InProgress;
use std::io::Write;
use ui::color::Color;
use ui::input::Input;
use ui::view::View;
use ui::*;

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
