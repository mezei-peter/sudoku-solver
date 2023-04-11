use std::vec::Vec;

pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<char>>
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<char>>) -> Puzzle {
        Puzzle { grid_size, matrix }
    }
}