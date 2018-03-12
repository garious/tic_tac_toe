extern crate rand;

use rand::Rng;
use board::Board;
use strategy::Strategy;

#[derive(Debug, PartialEq)]
pub struct Lazy;

impl Lazy {
    pub fn new() -> Lazy {
        Lazy {}
    }
}

impl Strategy for Lazy {
    fn decide(&self, board: &Board) -> usize {
        let empty_cells = board.empty_cells();
        let random_number = rand::thread_rng().gen_range(0, empty_cells.len());

        empty_cells[random_number]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::*;

    #[test]
    fn it_picks_random_empty_cell() {
        let strategy = Lazy::new();
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        let board = create_board_filling_cells(3, fill_spots);
        let selection = strategy.decide(&board);

        assert!(empty_spots.contains(&selection));
    }
}
