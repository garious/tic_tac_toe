#[allow(dead_code)]
pub enum Color {
    Dim,
    Normal,
}

impl Color {
    pub fn fill(&self, text: &str) -> String {
        match *self {
            Color::Dim => format!("\x1B[2m{}\x1B[0m", text),
            Color::Normal => format!("{}", text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_dimmed_string() {
        assert_eq!("\x1B[2mStuff\x1b[0m", Color::Dim.fill("Stuff"));
    }

    #[test]
    fn it_returns_normal_string() {
        assert_eq!("Stuff", Color::Normal.fill("Stuff"));
    }
}
