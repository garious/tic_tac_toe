use std::io::{BufRead, Write};
use board::*;
use io::*;

#[derive(Debug, PartialEq)]
pub struct Player {
    token: CellState,
}

impl Player {
    pub fn new(token: CellState) -> Player {
        Player { token }
    }

    pub fn get_token(&self) -> &CellState {
        &self.token
    }

    pub fn get_move<R: BufRead, W: Write>(&self, board: &Board, io: &mut IO<R, W>) -> usize {
        let range = board.get_size().pow(2);
        let prompt = format!("Pick a spot between 1-{}", range);
        let selection = io.prompt(&prompt);

        let selection: usize = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                io.print("Invavlid selection.");
                self.get_move(board, io)
            }
        };

        selection - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::CellState::*;

    #[test]
    fn it_creates_new_player() {
        let player = Player::new(Cross);
        assert_eq!(Cross, player.token);
    }

    #[test]
    fn it_gets_player_token() {
        let player = Player::new(Cross);
        assert_eq!(&Cross, player.get_token());
    }

    #[test]
    fn it_gets_player_move() {
        let player = Player::new(Cross);
        let board = Board::new(3);
        let mut io = IO::new(&b"1"[..], Vec::new(), "");
        let selection = player.get_move(&board, &mut io);

        assert_eq!(0, selection);
    }
}
