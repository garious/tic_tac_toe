extern crate rand;

use rand::Rng;
use board::*;
use strategy::*;

pub struct Lazy;

impl Lazy {
    pub fn new() -> Lazy {
        Lazy {}
    }
}

impl Strategy for Lazy {
    fn decide(&self, _: CellState, board: &Board) -> usize {
        let empty_cells = board.empty_cells();
        let random_number = rand::thread_rng().gen_range(0, empty_cells.len());

        empty_cells[random_number]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::CellState::*;

    fn update_cells(indices: Vec<usize>, board: &mut Board) {
        for i in indices.iter() {
            match i {
                i if i % 2 == 0 => board.update(*i, Cross),
                _ => board.update(*i, Nought),
            }
        }
    }

    #[test]
    fn it_picks_random_empty_cell() {
        let strategy = Lazy::new();
        let mut board = Board::new(3);
        let fill_spots = vec![0, 1, 3, 4];
        let empty_spots = vec![2, 5, 6, 7, 8];
        update_cells(fill_spots, &mut board);
        let selection = strategy.decide(Cross, &board);

        assert!(empty_spots.contains(&selection));
    }
}
