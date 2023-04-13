use std::vec::Vec;

use crate::model::cell::Cell;

#[derive(Debug)]
pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<Cell>>,
    current_x: u8,
    current_y: u8,
    initial_pos: bool,
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<Cell>>) -> Puzzle {
        Puzzle {
            grid_size,
            matrix,
            current_x: 0,
            current_y: 0,
            initial_pos: true,
        }
    }

    pub fn next_cell(&mut self) -> Result<&mut Cell, ()> {
        if self.initial_pos {
            self.initial_pos = false;
            return Ok(&mut self.matrix[0][0]);
        }

        if self.current_y < self.grid_size - 1 {
            self.current_y += 1;
        } else if self.current_y == self.grid_size - 1 {
            self.current_y = 0;
            self.current_x += 1;
        }

        if self.current_x == self.grid_size {
            return Err(());
        }

        Ok(&mut self.matrix[self.current_x as usize][self.current_y as usize])
    }
}

impl Clone for Puzzle {
    fn clone(&self) -> Puzzle {
        Puzzle::new(self.grid_size.clone(), self.matrix.clone())
    }
}
