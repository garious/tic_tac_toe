use std::io::BufRead;
use board::{Board, CellState};
use player::Player;
use script::Script::InvalidSelection;
use user_input::UserInput;

const TO_INDEX: usize = 1;

#[derive(Debug, PartialEq)]
pub struct Human<R> {
    token: CellState,
    input: UserInput<R>,
}

impl<R: BufRead> Human<R> {
    pub fn new(token: CellState, input: UserInput<R>) -> Human<R> {
        Human { token, input }
    }
}

impl<R: BufRead> Player for Human<R> {
    fn get_token(&self) -> &CellState {
        &self.token
    }

    fn get_move(&mut self, _: &Board) -> Result<usize, String> {
        let selection = self.input.read_line();
        let result: Result<usize, String> = match selection.trim().parse::<usize>() {
            Ok(num) => Ok(num - TO_INDEX),
            Err(_) => Err(String::from(InvalidSelection.to_str())),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::CellState::Cross;

    #[test]
    fn it_creates_new_player() {
        let user_input = UserInput::new(&b"1"[..]);
        let player = Human::new(Cross, user_input);
        assert_eq!(Cross, player.token);
    }

    #[test]
    fn it_gets_player_token() {
        let user_input = UserInput::new(&b"1"[..]);
        let player = Human::new(Cross, user_input);
        assert_eq!(&Cross, player.get_token());
    }

    #[test]
    fn it_gets_player_move() {
        let user_input = UserInput::new(&b"1"[..]);
        let mut player = Human::new(Cross, user_input);
        let board = Board::new(3);
        let selection = player.get_move(&board).unwrap();

        assert_eq!(0, selection);
    }
}
