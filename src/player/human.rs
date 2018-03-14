use board::Board;
use player::Player;
use script::Script::InvalidSelection;
use token::Token;
use ui::input::Input;

const TO_INDEX: usize = 1;

#[derive(Debug, PartialEq)]
pub struct Human<I> {
    token: Token,
    input: I,
}

impl<I: Input> Human<I> {
    pub fn new(token: Token, input: I) -> Human<I> {
        Human { token, input }
    }
}

impl<I: Input> Player for Human<I> {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_move(&mut self, board: &Board) -> Result<usize, String> {
        let selection = self.input.read_line();
        let result: Result<usize, String> = match selection.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= board.get_length() => Ok(num - TO_INDEX),
            Ok(_) | Err(_) => Err(String::from(InvalidSelection.to_str())),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::Cross;
    use ui::input::tests::*;

    #[test]
    fn it_creates_new_player() {
        let mock_input = MockInput::new(vec!["1"]);
        let player = Human::new(Cross, mock_input);
        assert_eq!(Cross, player.token);
    }

    #[test]
    fn it_gets_player_token() {
        let mock_input = MockInput::new(vec!["1"]);
        let player = Human::new(Cross, mock_input);
        assert_eq!(&Cross, player.get_token());
    }

    #[test]
    fn it_gets_player_move() {
        let mock_input = MockInput::new(vec!["1"]);
        let mut player = Human::new(Cross, mock_input);
        let board = Board::new(3);
        let selection = player.get_move(&board);

        assert_eq!(Ok(0), selection);
    }

    #[test]
    fn it_returns_error_for_nonnumeric_input() {
        let mock_input = MockInput::new(vec!["y"]);
        let mut player = Human::new(Cross, mock_input);
        let board = Board::new(3);
        let selection = player.get_move(&board);
        let expected = Err(String::from(InvalidSelection.to_str()));

        assert_eq!(expected, selection);
    }

    #[test]
    fn it_returns_error_for_above_range_input() {
        let mock_input = MockInput::new(vec!["10"]);
        let mut player = Human::new(Cross, mock_input);
        let board = Board::new(3);
        let selection = player.get_move(&board);
        let expected = Err(String::from(InvalidSelection.to_str()));

        assert_eq!(expected, selection);
    }

    #[test]
    fn it_returns_error_for_below_range_input() {
        let mock_input = MockInput::new(vec!["0"]);
        let mut player = Human::new(Cross, mock_input);
        let board = Board::new(3);
        let selection = player.get_move(&board);
        let expected = Err(String::from(InvalidSelection.to_str()));

        assert_eq!(expected, selection);
    }
}
