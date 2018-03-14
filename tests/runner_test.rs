extern crate assert_cli;
extern crate ttt_lib;

#[cfg(test)]
mod tests {
    use assert_cli;
    use ttt_lib::board::Board;
    use ttt_lib::script::Script::*;
    use ttt_lib::token::Token::*;
    use ttt_lib::ui::color::Color::Dim;
    use ttt_lib::ui::presenter;

    fn get_board_patterns(indices: Vec<usize>) -> Vec<String> {
        let mut patterns = Vec::new();
        let mut board = Board::new(3);
        for i in indices.iter() {
            let token = match i % 2 != 0 {
                true => Cross,
                false => Nought,
            };
            board = board.update(*i - 1, token);
            let pattern = presenter::view(&board, &Dim);
            patterns.push(pattern);
        }

        patterns
    }

    #[test]
    fn it_runs_computer_vs_computer_game() {
        assert_cli::Assert::main_binary()
            .stdin("\n4\nn\n")
            .stdout()
            .contains(Welcome.to_str())
            .stdout()
            .contains(ModeSelection.to_str())
            .stdout()
            .contains("[Player X] ~ Pick an open spot between 1-9.")
            .stdout()
            .contains("[Player O] ~ Pick an open spot between 1-9.")
            .stdout()
            .contains(Draw.to_str())
            .stdout()
            .contains(PlayAgain.to_str())
            .stdout()
            .contains(Goodbye.to_str())
            .unwrap();
    }

    #[test]
    fn it_runs_human_vs_human_game() {
        let patterns = get_board_patterns(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_cli::Assert::main_binary()
            .stdin("\n1\n1\n2\n3\n4\n5\n6\n7\nn\n")
            .stdout()
            .contains(Welcome.to_str())
            .stdout()
            .contains(ModeSelection.to_str())
            .stdout()
            .contains("[Player X] ~ Pick an open spot between 1-9.")
            .stdout()
            .contains("[Player O] ~ Pick an open spot between 1-9.")
            .stdout()
            .contains(&*patterns[0])
            .stdout()
            .contains(&*patterns[1])
            .stdout()
            .contains(&*patterns[2])
            .stdout()
            .contains(&*patterns[3])
            .stdout()
            .contains(&*patterns[4])
            .stdout()
            .contains(&*patterns[5])
            .stdout()
            .contains(&*patterns[6])
            .stdout()
            .contains("X wins!!!")
            .stdout()
            .contains(PlayAgain.to_str())
            .stdout()
            .contains(Goodbye.to_str())
            .unwrap();
    }
}
