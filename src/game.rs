use board::Board;
use player::Player;
use rules;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Over,
}

pub struct Game {
    board: Board,
    player_one: Box<dyn Player>,
    player_two: Box<dyn Player>,
    state: GameState,
}

impl Game {
    pub fn new(board: Board, player_one: Box<dyn Player>, player_two: Box<dyn Player>) -> Game {
        Game {
            board,
            player_one,
            player_two,
            state: GameState::InProgress,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_winner(&self) -> &Token {
        rules::get_winner(&self.board)
    }

    pub fn next_turn(&mut self) {
        let token = self.current_player_token();
        let move_choice = self.current_player_move();
        let cells = self.board.clone();

        self.board = match move_choice {
            Ok(num) => cells.update(num, token),
            Err(_) => cells,
        };

        self.update_state();
    }

    pub fn current_player_token(&self) -> Token {
        match self.is_odd_turn() {
            true => *self.player_one.get_token(),
            false => *self.player_two.get_token(),
        }
    }

    fn current_player_move(&mut self) -> Result<usize, String> {
        match self.is_odd_turn() {
            true => self.player_one.get_move(&self.board),
            false => self.player_two.get_move(&self.board),
        }
    }

    fn is_odd_turn(&self) -> bool {
        let number_empty_cells = self.board.empty_cells().len();
        number_empty_cells % 2 != 0
    }

    fn update_state(&mut self) {
        if rules::is_game_over(&self.board) {
            self.state = GameState::Over;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::GameState::{InProgress, Over};
    use super::*;
    use board::tests::*;
    use board::Board;
    use player::computer::Computer;
    use player::human::Human;
    use player::strategy::lazy::Lazy;
    use token::Token::{Cross, Nought};
    use ui::input::tests::*;

    pub fn setup_computer_vs_computer(board: Board) -> Game {
        let player_one = Box::new(Computer::new(Cross, Lazy::default()));
        let player_two = Box::new(Computer::new(Nought, Lazy::default()));
        Game::new(board, player_one, player_two)
    }

    fn setup_human_vs_computer() -> Game {
        let board = Board::new(3);
        let mock_input = MockInput::new(vec!["1"]);
        let player_one = Box::new(Human::new(Cross, mock_input));
        let player_two = Box::new(Computer::new(Nought, Lazy::default()));
        Game::new(board, player_one, player_two)
    }

    #[test]
    fn it_creates_new_game() {
        let game = setup_human_vs_computer();
        assert_eq!(Board::new(3), game.board);
        assert_eq!(Cross, *game.player_one.get_token());
        assert_eq!(Nought, *game.player_two.get_token());
        assert_eq!(InProgress, game.state);
    }

    #[test]
    fn it_gets_game_board() {
        let game = setup_human_vs_computer();
        assert_eq!(&Board::new(3), game.get_board());
    }

    #[test]
    fn it_gets_game_state() {
        let game = setup_human_vs_computer();
        assert_eq!(&InProgress, game.get_state());
    }

    #[test]
    fn it_gets_game_winner() {
        let board = create_patterned_board(3, (0..9).collect());
        let game = setup_computer_vs_computer(board);

        assert_eq!(&Cross, game.get_winner());
    }

    #[test]
    fn it_progresses_game() {
        let mut game = setup_computer_vs_computer(Board::new(3));
        let number_turns = 9;

        assert_eq!(InProgress, game.state);

        for i in 0..number_turns {
            assert_eq!(number_turns - i, game.board.empty_cells().len());
            game.next_turn();
        }

        assert_eq!(Over, game.state);
    }
}
