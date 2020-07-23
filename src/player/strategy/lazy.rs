extern crate rand;

use board::Board;
use player::strategy::Strategy;
use rand::Rng;

#[derive(Debug, Default, PartialEq)]
pub struct Lazy;

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
        let strategy = Lazy::default();
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        let board = create_patterned_board(3, fill_spots);
        let selection = strategy.decide(&board);

        assert!(empty_spots.contains(&selection));
    }
}
