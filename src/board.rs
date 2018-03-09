use token::Token;
use token::Token::*;

const MODIFIER: usize = 1;

type CellMatrix = Vec<Vec<Token>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    size: usize,
    cells: Vec<Token>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![Empty; (size * size)],
        }
    }

    pub fn get_cells(&self) -> &Vec<Token> {
        &self.cells
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn update(&mut self, cell_move: usize, token: Token) {
        self.cells[cell_move] = token;
    }

    pub fn partition(&self) -> CellMatrix {
        let rows = self.rows();
        let columns = self.columns(&rows);
        let diagonals = self.diagonals(&rows);

        [&rows[..], &columns[..], &diagonals[..]].concat()
    }

    pub fn is_empty_cell(&self, index: usize) -> bool {
        match self.cells[index] {
            Empty => true,
            _ => false,
        }
    }

    pub fn empty_cells(&self) -> Vec<usize> {
        let mut empty_cells = Vec::new();

        for (i, _) in self.cells.iter().enumerate() {
            if self.is_empty_cell(i) {
                empty_cells.push(i);
            }
        }

        empty_cells
    }

    fn rows(&self) -> CellMatrix {
        self.cells.chunks(self.size).map(|x| x.to_vec()).collect()
    }

    fn columns(&self, rows: &CellMatrix) -> CellMatrix {
        let mut columns = rows.to_vec();

        for i in 0..self.size {
            for j in i + MODIFIER..self.size {
                let temp = columns[i][j];
                columns[i][j] = columns[j][i];
                columns[j][i] = temp;
            }
        }

        columns
    }

    fn diagonals(&self, rows: &CellMatrix) -> CellMatrix {
        let left_diagonal = self.left_diagonal(rows);
        let right_diagonal = self.right_diagonal(rows);

        vec![left_diagonal, right_diagonal]
    }

    fn left_diagonal(&self, rows: &CellMatrix) -> Vec<Token> {
        rows.iter().enumerate().map(|(i, x)| x[i]).collect()
    }

    fn right_diagonal(&self, rows: &CellMatrix) -> Vec<Token> {
        let row_end = self.size - MODIFIER;

        rows.iter()
            .enumerate()
            .map(|(i, x)| x[(row_end - i)])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn update_cells(indices: Vec<usize>, board: &mut Board) {
        for i in indices.iter() {
            match i {
                i if i % 2 == 0 => board.update(*i, Cross),
                _ => board.update(*i, Nought),
            }
        }
    }

    #[test]
    fn it_creates_empty_board() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(vec![Empty; (size * size)], board.cells);
        assert_eq!(3, board.size);
    }

    #[test]
    fn it_gets_board_cells() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(board.cells, *board.get_cells())
    }

    #[test]
    fn it_gets_size_of_board() {
        let size = 3;
        let board = Board::new(size);
        assert_eq!(board.size, board.get_size());
    }

    #[test]
    fn it_sets_board_cell_state() {
        let size = 3;
        let mut board = Board::new(size);
        update_cells(vec![0, 5], &mut board);
        assert_eq!(Cross, board.cells[0]);
        assert_eq!(Nought, board.cells[5]);
    }

    #[test]
    fn it_informs_if_cell_is_empty() {
        let mut board = Board::new(3);
        update_cells(vec![6], &mut board);
        assert_eq!(false, board.is_empty_cell(6));
        assert!(board.is_empty_cell(0));
    }

    #[test]
    fn it_gets_indices_of_empty_cells() {
        let mut board = Board::new(3);
        update_cells(vec![0, 2, 5, 7], &mut board);
        assert_eq!(vec![1, 3, 4, 6, 8], board.empty_cells());
    }

    #[test]
    fn it_partitions_board_into_rows_diagonals_columns() {
        let mut board = Board::new(3);
        update_cells(vec![0, 2, 3, 4, 7, 8], &mut board);
        let rows = vec![
            vec![Cross, Empty, Cross],
            vec![Nought, Cross, Empty],
            vec![Empty, Nought, Cross],
        ];
        let columns = vec![
            vec![Cross, Nought, Empty],
            vec![Empty, Cross, Nought],
            vec![Cross, Empty, Cross],
        ];
        let diagonals = vec![vec![Cross, Cross, Cross], vec![Cross, Cross, Empty]];
        let partition = [&rows[..], &columns[..], &diagonals[..]].concat();

        assert_eq!(partition, board.partition());
    }
}
