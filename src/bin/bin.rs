extern crate ttt_lib;

use ttt_lib::run;
use ttt_lib::ui::input::UserInput;
use ttt_lib::ui::view::View;

fn main() {
    let output = std::io::stdout();
    let user_input = UserInput::new();
    let mut view = View::new(output);

    run(user_input, &mut view);
}
