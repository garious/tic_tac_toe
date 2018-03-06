use std::io::{BufRead, Write};

pub struct IO<'a, R, W> {
    reader: R,
    writer: W,
    clear_sequence: &'a str,
}

impl<'a, R, W> IO<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W, clear_sequence: &'a str) -> IO<'a, R, W> {
        IO {
            reader,
            writer,
            clear_sequence: clear_sequence,
        }
    }

    pub fn print(&mut self, message: &str) {
        self.clear();
        write!(&mut self.writer, "{}\n", message).expect("Unable to write");
    }

    pub fn prompt(&mut self, prompt: &str) -> String {
        self.clear();
        self.print(prompt);
        self.read_line()
    }

    fn clear(&mut self) {
        write!(&mut self.writer, "{}", self.clear_sequence).expect("Unable to write");
    }

    fn read_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Unable to read");
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_io() {
        let input = b"1";
        let output = Vec::new();
        let clear = "clear";
        let io = IO::new(&input[..], output.clone(), &clear);

        assert_eq!(input, io.reader);
        assert_eq!(output, io.writer);
        assert_eq!(clear, io.clear_sequence);
    }

    #[test]
    fn it_prints_to_writer() {
        let input = b"1";
        let output = Vec::new();
        let mut io = IO::new(&input[..], output, "");
        io.print("Tic Tac Toe");
        let output = String::from_utf8(io.writer).expect("Not UTF-8");

        assert_eq!("Tic Tac Toe\n", output);
    }

    #[test]
    fn it_gets_prompt_response() {
        let input = b"1";
        let output = Vec::new();
        let mut io = IO::new(&input[..], output, "");
        let response = io.prompt("Tic Tac Toe");
        let output = String::from_utf8(io.writer).expect("Not UTF-8");

        assert_eq!("Tic Tac Toe\n", output);
        assert_eq!("1", response);
    }
}
