use board::Board;
use game::Game;
use player::*;
use player::computer::Computer;
use player::human::Human;
use player::strategy::lazy::Lazy;
use player::strategy::unbeatable::Unbeatable;
use token::Token::{Cross, Nought};
use script::Script::ModeSelection;
use std::io::Write;
use ui::input::Input;
use ui::view::View;
use ui::input::UserInput;

const MODE_OPTIONS: [u32; 4] = [1, 2, 3, 4];

pub fn select_mode<I: Input, W: Write>(user_input: &mut I, view: &mut View<W>) -> u32 {
    view.update_with(ModeSelection.to_str());
    match user_input.read_line().trim().parse() {
        Ok(num) if MODE_OPTIONS.contains(&num) => num,
        Ok(_) | Err(_) => select_mode(user_input, view),
    }
}

pub fn setup_game(mode_selection: u32) -> Game {
    let board = Board::new(3);
    let (player_one, player_two) = setup_players(mode_selection);
    Game::new(board, player_one, player_two)
}

fn setup_players(mode_selection: u32) -> (Box<Player>, Box<Player>) {
    match mode_selection {
        1 => (
            Box::new(Human::new(Cross, UserInput::new())),
            Box::new(Human::new(Nought, UserInput::new())),
        ),
        2 => (
            Box::new(Human::new(Cross, UserInput::new())),
            Box::new(Computer::new(Nought, Lazy::new())),
        ),
        3 => (
            Box::new(Human::new(Cross, UserInput::new())),
            Box::new(Computer::new(Nought, Unbeatable::new(Nought))),
        ),
        _ => (
            Box::new(Computer::new(Cross, Unbeatable::new(Cross))),
            Box::new(Computer::new(Nought, Unbeatable::new(Nought))),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::input::tests::*;

    #[test]
    fn it_keeps_prompting_for_valid_mode_option() {
        let mut mock_input = MockInput::new(vec!["n", "5", "0", "-1", "", " ", "1"]);
        let mut view = View::new(Vec::new());
        let selection = select_mode(&mut mock_input, &mut view);
        assert_eq!(1, selection);
        assert_eq!(7, mock_input.times_called());
    }
}
