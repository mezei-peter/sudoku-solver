use std::vec::Vec;

#[derive(Debug)]
pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<char>>) -> Puzzle {
        Puzzle { grid_size, matrix }
    }

    pub fn clone(&self, original: &Puzzle) -> Puzzle {
        Puzzle::new(original.grid_size, original.matrix.to_vec())
    }
}
