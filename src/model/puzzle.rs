use std::vec::Vec;

use crate::model::cell::Cell;

#[derive(Debug)]
pub struct Puzzle {
    grid_size: u8,
    matrix: Vec<Vec<Cell>>,
}

impl Puzzle {
    pub fn new(grid_size: u8, matrix: Vec<Vec<Cell>>) -> Puzzle {
        Puzzle {
            grid_size,
            matrix,
        }
    }

    pub fn get_matrix_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let x_res = self.matrix.get(x);
        if x_res.is_none() {
            return None;
        }
        x_res.unwrap().get(y)
    }

    pub fn get_grid_size(&self) -> u8 {
        self.grid_size
    }

    pub fn replace_cell_value_at_position(&mut self, position: (u8, u8), value: char) {
        let line = self.matrix.get_mut(position.0 as usize).unwrap();
        let cell: &mut Cell = line.get_mut(position.1 as usize).unwrap();
        cell.set_value(value);
    }

    pub fn valid_value_at_position(&self, value: char, position: (u8, u8)) -> bool {
        self.none_in_row(position.0 as usize, value)
            && self.none_in_column(position.1 as usize, value)
            && self.none_in_subgrid(position, value)
    }

    fn none_in_row(&self, x: usize, value: char) -> bool {
        let row = self.matrix.get(x).unwrap();
        let mut count: u8 = 0;
        row.iter().for_each(|cell: &Cell| {
            if cell.has_value(value) {
                count += 1;
            }
        });

        count == 0
    }

    fn none_in_column(&self, y: usize, value: char) -> bool {
        let mut column: Vec<&Cell> = Vec::<&Cell>::new();
        for row in &self.matrix {
            column.push(row.get(y).unwrap());
        }
        let mut count: u8 = 0;
        column.iter().for_each(|cell| {
            if cell.has_value(value) {
                count += 1;
            }
        });
        count == 0
    }

    fn none_in_subgrid(&self, position: (u8, u8), value: char) -> bool {
        let mut subgrid: Vec<&Cell> = Vec::<&Cell>::new();
        let (x, y) = position;
        let subgrid_y_size: u8 = f32::sqrt(self.grid_size as f32).ceil() as u8;
        let subgrid_x_size: u8 = f32::sqrt(self.grid_size as f32).floor() as u8;

        let (bottom_y, top_y) = self.find_border_values(y, subgrid_y_size);
        let (bottom_x, top_x) = self.find_border_values(x, subgrid_x_size);

        for i in bottom_x..top_x + 1 {
            for j in bottom_y..top_y + 1 {
                subgrid.push(self.get_matrix_cell(i as usize, j as usize).unwrap());
            }
        }

        let mut count: u8 = 0;
        subgrid.iter().for_each(|cell| {
            if cell.has_value(value) {
                count += 1;
            }
        });
        count == 0
    }

    fn find_border_values(&self, coordinate: u8, increment: u8) -> (u8, u8) {
        let mut top_val: u8 = 0;

        loop {
            if coordinate < top_val {
                return (top_val - increment, top_val - 1);
            }
            top_val += increment;
        }
    }

    pub fn clone_matrix(&self) -> Vec<Vec<Cell>> {
        self.matrix.clone()
    }
}

impl Clone for Puzzle {
    fn clone(&self) -> Puzzle {
        Puzzle::new(self.grid_size.clone(), self.matrix.clone())
    }
}
