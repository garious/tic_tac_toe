#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Empty,
    Cross,
    Nought,
}

impl Token {
    pub fn to_str(&self) -> &str {
        match *self {
            Token::Empty => " ",
            Token::Cross => "X",
            Token::Nought => "O",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token::*;

    #[test]
    fn it_gets_string_for_token() {
        assert_eq!(" ", Empty.to_str());
        assert_eq!("X", Cross.to_str());
        assert_eq!("O", Nought.to_str());
    }
}
