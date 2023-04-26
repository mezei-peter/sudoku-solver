
use crate::model::{puzzle::{Puzzle, self}, cell::Cell, default_puzzle_properties::DefaultProps};

pub trait FormatConverter {
    fn puzzle_to_ss(&self, puzzle: &Puzzle) -> String;
}

pub struct FormatConverterImpl;

impl FormatConverterImpl {
    pub fn new() -> FormatConverterImpl {
        FormatConverterImpl
    }
}

impl FormatConverter for FormatConverterImpl {
    fn puzzle_to_ss(&self, puzzle: &Puzzle) -> String {
        let mut ss: String = String::new();
        let grid_size: u8 = puzzle.get_grid_size();
        let subgrid_width: u8 = f32::sqrt(grid_size as f32).ceil() as u8;
        let subgrid_height: u8 = f32::sqrt(grid_size as f32).floor() as u8;
        let matrix: Vec<Vec<Cell>> = puzzle.clone_matrix();

        for row in matrix {
            for cell in row {
                let y: u8 = cell.get_pos_y();
                let x: u8 = cell.get_pos_x();

                if cell.is_empty() {
                    ss.push('.');
                } else {
                    ss.push(cell.get_value());
                }
                if y != 0 && y != grid_size - 1 {
                    if (y + 1) % subgrid_width == 0 {
                        ss.push('|');
                    }
                }

                if y == grid_size - 1 {
                    ss.push('\n');
                    if x != 0 && x != grid_size - 1 {
                        if (x + 1) % subgrid_height == 0 {
                            ss.push_str("-".repeat(grid_size as usize + subgrid_width as usize - 1).as_str());
                            ss.push('\n');
                        }
                    }
                }
            }
        }
        ss
    }
}
