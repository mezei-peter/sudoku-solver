use std::vec::Vec;

use crate::model::cell::Cell;

#[derive(Debug)]
pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<Cell>>,
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<Cell>>) -> Puzzle {
        Puzzle { grid_size, matrix }
    }

}

impl Clone for Puzzle {
    fn clone(&self) -> Puzzle {
        Puzzle::new(self.grid_size.clone(), self.matrix.clone())
    }
}
