use board::Board;
use player::strategy::Strategy;
use player::Player;
use token::Token;

#[derive(Debug, PartialEq)]
pub struct Computer<S> {
    token: Token,
    strategy: S,
}

impl<S: Strategy> Computer<S> {
    pub fn new(token: Token, strategy: S) -> Computer<S> {
        Computer { token, strategy }
    }
}

impl<S: Strategy> Player for Computer<S> {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_move(&mut self, board: &Board) -> Result<usize, String> {
        Ok(self.strategy.decide(board))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::*;
    use player::strategy::lazy::Lazy;
    use token::Token::Cross;

    #[test]
    fn it_creates_new_player() {
        let player = Computer::new(Cross, Lazy::default());
        assert_eq!(Cross, player.token);
        assert_eq!(Lazy::default(), player.strategy);
    }

    #[test]
    fn it_gets_player_token() {
        let player = Computer::new(Cross, Lazy::default());
        assert_eq!(&Cross, player.get_token());
    }

    #[test]
    fn it_gets_player_move() {
        let mut player = Computer::new(Cross, Lazy::default());
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        let board = create_patterned_board(3, fill_spots);
        let selection = player.get_move(&board).unwrap();

        assert!(empty_spots.contains(&selection));
    }
}
