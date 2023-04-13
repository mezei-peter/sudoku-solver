use std::vec::Vec;

use crate::model::cell::Cell;

#[derive(Debug)]
pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<char>>) -> Puzzle {
        Puzzle { grid_size, matrix }
    }

    pub fn clone(&self) -> Puzzle {
        Puzzle::new(self.grid_size, self.matrix.to_vec())
    }

    pub fn map_to_cell_matrix(&self) -> Vec<Vec<Cell>> {
        self.matrix
            .iter()
            .map(|row| row
                .iter()
                .map(|ch: &char| Cell::new(ch.clone(), if *ch == '0' { false } else { true }))
                .collect()
            )
            .collect()
    }
}
