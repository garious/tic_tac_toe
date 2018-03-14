pub enum Script {
    Welcome,
    ModeSelection,
    PickSpot,
    InvalidSelection,
    Draw,
    Wins,
    PlayAgain,
    Goodbye,
}

impl Script {
    pub fn to_str(&self) -> &str {
        match *self {
            Script::Welcome => {
                "+--------------------------------+ \
                 \n| ~~~~~ Rusted Tic Tac Toe ~~~~~ | \
                 \n+--------------------------------+\n\
                 \nTic tac toe is a game of matching.\
                 \nFill a row, column, or diagonal\
                 \nwith your token and you win. But\
                 \nif you opponent does the same,\
                 \nthey win. So watch out!\
                 \n\nPress [enter] to play."
            }
            Script::ModeSelection => {
                "Select a game mode:\n\
                 \n    [1] human vs. human,\
                 \n    [2] human vs. computer (easy),\
                 \n    [3] human vs. computer (impossible),\
                 \n    [4] computer vs. computer.
                 \nSelection:"
            }
            Script::PickSpot => "Pick an open spot between 1-",
            Script::InvalidSelection => "Invalid selection.",
            Script::Draw => "It's a draw.",
            Script::Wins => " wins!!!",
            Script::PlayAgain => "\nWould you like to play again? [y/n]",
            Script::Goodbye => {
                "+---------------------------------+ \
                 \n| ~~~~~ Goodbye and thanks. ~~~~~ | \
                 \n+---------------------------------+\n"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Script::*;

    #[test]
    fn it_returns_correct_messages() {
        let pick_spot = "Pick an open spot between 1-";
        let invalid_selection = "Invalid selection.";
        assert_eq!(pick_spot, PickSpot.to_str());
        assert_eq!(invalid_selection, InvalidSelection.to_str());
    }
}
