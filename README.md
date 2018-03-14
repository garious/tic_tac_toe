# Rusted Tic Tac Toe

An unbeatable game of tic tac toe built in Rust.

## Requirements

* Rust 1.24.1
* Cargo 0.25.0

## Playing the game

`git clone git@github.com:saralein/tic_tac_toe.git`
`cd tic_tac_toe`
`cargo run`

## Building/running release version

You may also build and run a release version of the game with the following:

`cargo build --release`
`./target/release/ttt_bin`

I recommend trying this out with a computer vs. computer game. Then compare speeds with a `cargo run` computer vs. computer game. It is pretty cool.

## Running the tests

`cargo test` will run the tests. Please note that unit tests are placed below the code in each `src` file per Rust conventions. Integration tests appear in `tests`. `cargo test` runs both.
