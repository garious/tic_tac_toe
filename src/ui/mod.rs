pub mod color;
pub mod presenter;
pub mod input;
pub mod script;
pub mod view;

use game::Game;
use script::Script::*;
use std::io::Write;
use token::Token::Empty;
use ui::color::Color;
use ui::input::Input;
use ui::view::View;

const PLAY_AGAIN: &str = "y";

pub fn welcome<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) {
    view.update_with(Welcome.to_str());
    user_input.read_line();
}

pub fn prompt_turn<W: Write>(game: &mut Game, view: &mut View<W>, color: &Color) {
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

pub fn show_winner<W: Write>(game: &mut Game, view: &mut View<W>, color: &Color) {
    let board = game.get_board();
    let winner = game.get_winner();

    view.update_with(&presenter::view(&board, color));

    match winner {
        &Empty => view.append_with(Draw.to_str()),
        _ => view.append_with(&format!("{}{}", winner.to_str(), Wins.to_str())),
    };
}

pub fn select_exit<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) -> bool {
    view.append_with(PlayAgain.to_str());
    user_input.read_line().trim() != PLAY_AGAIN
}

pub fn goodbye<W: Write>(view: &mut View<W>) {
    view.update_with(Goodbye.to_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::*;
    use color::Color::Normal;
    use game::tests::*;
    use ui::input::tests::MockInput;

    #[test]
    fn it_prompts_turn() {
        let board = create_patterned_board(3, (0..8).collect());
        let mut game = setup_computer_vs_computer(board);
        let mut view = View::new(Vec::new());
        prompt_turn(&mut game, &mut view, &Normal);
        let output = String::from_utf8(view.get_writer().clone()).expect("Not UTF-8");
        let board_display = " X | O | X \n---+---+---\n O | X | O \n---+---+---\n X | O | 9 ";
        assert!(output.contains(board_display));
        assert!(output.contains("[Player X] ~ Pick an open spot between 1-9."));
    }

    #[test]
    fn it_shows_winner() {
        let board = create_patterned_board(3, (0..9).collect());
        let mut game = setup_computer_vs_computer(board);
        let mut view = View::new(Vec::new());
        show_winner(&mut game, &mut view, &Normal);
        let output = String::from_utf8(view.get_writer().clone()).expect("Not UTF-8");
        let board_display = " X | O | X \n---+---+---\n O | X | O \n---+---+---\n X | O | X ";
        assert!(output.contains(board_display));
        assert!(output.contains("X wins!!!"));
    }

    #[test]
    fn it_shows_a_draw() {
        let board = create_tied_board(3);
        let mut game = setup_computer_vs_computer(board);
        let mut view = View::new(Vec::new());
        show_winner(&mut game, &mut view, &Normal);
        let output = String::from_utf8(view.get_writer().clone()).expect("Not UTF-8");
        let board_display = " X | O | X \n---+---+---\n X | O | O \n---+---+---\n O | X | X ";
        assert!(output.contains(board_display));
        assert!(output.contains("It's a draw"));
    }

    #[test]
    fn it_relays_preference_to_play_again() {
        let input = vec!["n"];
        let mut mock_input = MockInput::new(input);
        let mut view = View::new(Vec::new());
        assert!(select_exit(&mut mock_input, &mut view));
    }

    #[test]
    fn it_relays_preference_to_exit() {
        let input = vec!["y"];
        let mut mock_input = MockInput::new(input);
        let mut view = View::new(Vec::new());
        assert_eq!(false, select_exit(&mut mock_input, &mut view));
    }
}
