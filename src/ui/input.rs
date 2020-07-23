use std::io::{stdin, BufRead};

pub trait Input {
    fn read_line(&mut self) -> String;
}

#[derive(Default)]
pub struct UserInput;

impl UserInput {
    fn read<R: BufRead>(&self, mut reader: R) -> String {
        let mut input = String::new();
        reader.read_line(&mut input).expect("Unable to read");
        input
    }
}

impl Input for UserInput {
    fn read_line(&mut self) -> String {
        let stdio = stdin();
        let input = stdio.lock();
        self.read(input)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct MockInput<'a> {
        input: Vec<&'a str>,
        called: usize,
    }

    impl<'a> Input for MockInput<'a> {
        fn read_line(&mut self) -> String {
            let index = self.called;
            self.called += 1;
            String::from(self.input[index])
        }
    }

    impl<'a> MockInput<'a> {
        pub fn new(input: Vec<&'a str>) -> MockInput<'a> {
            MockInput { input, called: 0 }
        }

        pub fn times_called(self) -> usize {
            self.called
        }
    }

    #[test]
    fn it_gets_user_response() {
        let input = b"1";
        let user_input = UserInput::default();

        assert_eq!("1", user_input.read(&input[..]));
    }
}
