use std::io::{stdin, BufRead, BufReader};

pub trait Input {
    fn read_line(&mut self) -> String;
}

pub struct UserInput {
    reader: Box<BufRead>,
}

impl UserInput {
    pub fn new() -> UserInput {
        let reader = BufReader::new(stdin());
        UserInput {
            reader: Box::new(reader),
        }
    }
}

impl Input for UserInput {
    fn read_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Unable to read");
        input
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct MockInput<'a> {
        input: &'a str,
    }

    impl<'a> Input for MockInput<'a> {
        fn read_line(&mut self) -> String {
            String::from(self.input)
        }
    }

    impl<'a> MockInput<'a> {
        pub fn new(input: &'a str) -> MockInput<'a> {
            MockInput { input }
        }
    }

    #[test]
    fn it_gets_user_response() {
        let input = b"1";
        let mut user_input = UserInput {
            reader: Box::new(&input[..]),
        };

        assert_eq!("1", user_input.read_line());
    }
}
