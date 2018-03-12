use std::io::Write;
use board::Board;
use player::Player;
use presenter;
use rules;
use script::Script::{Draw, PickSpot, Wins};
use token::Token;
use token::Token::*;
use view::View;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Over,
}

#[derive(Debug)]
pub struct Game<P, Q> {
    board: Board,
    player_one: P,
    player_two: Q,
    state: GameState,
}

impl<P: Player, Q: Player> Game<P, Q> {
    pub fn new(board: Board, player_one: P, player_two: Q) -> Game<P, Q> {
        Game {
            board,
            player_one,
            player_two,
            state: GameState::InProgress,
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn play<W: Write>(&mut self, view: &mut View<W>) {
        let board_length = self.board.get_length();
        self.reveal_board(view);
        view.print(&format!("{}{}", PickSpot.to_str(), board_length));

        self.take_turn();
        self.update_state();
    }

    pub fn reveal_winner<W: Write>(&mut self, view: &mut View<W>) {
        self.reveal_board(view);
        let winner = rules::get_winner(&self.board);

        match winner {
            &Empty => view.print(Draw.to_str()),
            _ => view.print(&format!("{}{}", winner.to_str(), Wins.to_str())),
        };
    }

    fn reveal_board<W: Write>(&mut self, view: &mut View<W>) {
        view.clear();
        view.print(&presenter::view(&self.board));
    }

    fn take_turn(&mut self) {
        let (move_result, token) = self.player_options();
        let cells = self.board.clone();

        self.board = match move_result {
            Ok(num) => cells.update(num, token),
            Err(_) => cells,
        };
    }

    fn player_options(&mut self) -> (Result<usize, String>, Token) {
        let number_empty_cells = self.board.empty_cells().len();
        let is_odd = number_empty_cells % 2 != 0;

        match is_odd {
            true => (
                self.player_one.get_move(&self.board),
                *self.player_one.get_token(),
            ),
            false => (
                self.player_two.get_move(&self.board),
                *self.player_two.get_token(),
            ),
        }
    }

    fn update_state(&mut self) {
        if rules::is_game_over(&self.board) {
            self.state = GameState::Over;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::GameState::{InProgress, Over};
    use board::Board;
    use board::tests::*;
    use player::computer::Computer;
    use player::human::Human;
    use player::strategy::lazy::Lazy;
    use user_input::UserInput;

    #[test]
    fn it_creates_new_game() {
        let board = Board::new(3);
        let user_input = UserInput::new(&b"1"[..]);
        let player_one = Human::new(Cross, user_input);
        let player_two = Computer::new(Nought, Lazy::new());
        let game = Game::new(board, player_one, player_two);
        let user_input = UserInput::new(&b"1"[..]);

        assert_eq!(Board::new(3), game.board);
        assert_eq!(Human::new(Cross, user_input), game.player_one);
        assert_eq!(Computer::new(Nought, Lazy::new()), game.player_two);
        assert_eq!(InProgress, game.state);
    }

    #[test]
    fn it_gets_game_state() {
        let board = Board::new(3);
        let user_input = UserInput::new(&b"1"[..]);
        let player_one = Human::new(Cross, user_input);
        let player_two = Computer::new(Nought, Lazy::new());
        let game = Game::new(board, player_one, player_two);

        assert_eq!(&InProgress, game.get_state());
    }

    #[test]
    fn it_progresses_game() {
        let board = Board::new(3);
        let mut view = View::new(Vec::new());
        let player_one = Computer::new(Cross, Lazy::new());
        let player_two = Computer::new(Nought, Lazy::new());
        let mut game = Game::new(board, player_one, player_two);
        let number_turns = 9;

        assert_eq!(InProgress, game.state);

        for i in 0..number_turns {
            assert_eq!(number_turns - i, game.board.empty_cells().len());
            game.play(&mut view);
        }

        assert_eq!(Over, game.state);
    }

    #[test]
    fn it_reveals_game_winner() {
        let board = create_board_filling_cells(3, (0..9).collect());
        let output = Vec::new();
        let mut view = View::new(output);
        let player_one = Computer::new(Cross, Lazy::new());
        let player_two = Computer::new(Nought, Lazy::new());
        let mut game = Game::new(board, player_one, player_two);
        game.reveal_winner(&mut view);
        let output = view.get_writer();
        let actual = String::from_utf8(output.clone()).expect("Not UTF-8");
        let divider = "\n---+---+---\n";
        let expected_board = vec![
            " X | O | X ",
            divider,
            " O | X | O ",
            divider,
            " X | O | X ",
        ].join("");

        assert!(&actual.contains(&expected_board));
        assert!(&actual.contains("X wins!"));
    }
}
