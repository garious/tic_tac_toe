use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct UserInput<R> {
    reader: R,
}

impl<R: BufRead> UserInput<R> {
    pub fn new(reader: R) -> UserInput<R> {
        UserInput { reader }
    }

    pub fn read_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Unable to read");
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_user_input() {
        let input = b"1";
        let user_input = UserInput::new(&input[..]);

        assert_eq!(&input[..], user_input.reader);
    }

    #[test]
    fn it_gets_user_response() {
        let input = b"1";
        let mut user_input = UserInput::new(&input[..]);

        assert_eq!("1", user_input.read_line());
    }
}
