pub enum Script {
    Welcome,
    HowToPlay,
    PickSpot,
    InvalidSelection,
}

impl Script {
    pub fn to_str(&self) -> &str {
        match *self {
            Script::Welcome => {
                "+--------------------------------+ \
                 \n| ~~~~~ Rusted Tic Tac Toe ~~~~~ | \
                 \n+--------------------------------+\n"
            }
            Script::HowToPlay => {
                "Tic tac toe is a game of matching.\
                 \nFill a row, column, or diagonal\
                 \nwith your token and you win. But\
                 \nif you opponent does the same,\
                 \nthey win. So watch out!\
                 \n\nPress [enter] to play."
            }
            Script::PickSpot => "Pick a spot between 1-",
            Script::InvalidSelection => "Invalid selection.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Script::*;

    #[test]
    fn it_returns_correct_messages() {
        let pick_spot = "Pick a spot between 1-";
        let invalid_selection = "Invalid selection.";
        assert_eq!(pick_spot, PickSpot.to_str());
        assert_eq!(invalid_selection, InvalidSelection.to_str());
    }
}
