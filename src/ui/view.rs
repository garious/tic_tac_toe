use std::io::Write;

pub struct View<W> {
    writer: W,
    clear_sequence: String,
}

impl<W: Write> View<W> {
    pub fn new(writer: W) -> View<W> {
        View {
            writer,
            clear_sequence: format!("{}[2J\n", 27 as char),
        }
    }

    #[allow(unused)]
    pub fn get_writer(&self) -> &W {
        &self.writer
    }

    pub fn print(&mut self, message: &str) {
        self.write(message);
    }

    pub fn clear_print(&mut self, message: &str) {
        self.clear();
        self.print(message);
    }

    pub fn clear(&mut self) {
        self.writer.flush().expect("Unable to flush");
        let clear = format!("{}", self.clear_sequence);
        self.write(&clear);
    }

    fn write(&mut self, message: &str) {
        write!(&mut self.writer, "{}\n", message).expect("Unable to write");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_view() {
        let output = Vec::new();
        let clear = format!("{}[2J\n", 27 as char);
        let view = View::new(output.clone());

        assert_eq!(output, view.writer);
        assert_eq!(clear, view.clear_sequence);
    }

    #[test]
    fn it_gets_writer() {
        let output = Vec::new();
        let view = View::new(output.clone());

        assert_eq!(output, *view.get_writer());
    }

    #[test]
    fn it_prints_messages() {
        let mut view = View::new(Vec::new());
        view.print("Tic Tac Toe");
        let output = String::from_utf8(view.writer).expect("Not UTF-8");

        assert_eq!("Tic Tac Toe\n", output);
    }

    #[test]
    fn it_clears_view() {
        let clear_sequence = String::from("clear");
        let mut view = View {
            writer: Vec::new(),
            clear_sequence: clear_sequence,
        };

        view.clear();
        let output = String::from_utf8(view.writer).expect("Not UTF-8");

        assert_eq!("clear\n", &output);
    }
}
