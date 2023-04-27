use crate::model::{cell::Cell, default_puzzle_properties::DefaultProps, puzzle::Puzzle};

use super::format_converter::{FormatConverter, FormatConverterImpl};

pub trait PuzzleEditor {
    fn create(&self) -> Puzzle;
}

pub struct PuzzleEditorImpl {
    format_converter: Box<dyn FormatConverter>,
}

enum CursorDirection {
    Forward,
    Back,
}

impl PuzzleEditorImpl {
    pub fn new() -> PuzzleEditorImpl {
        PuzzleEditorImpl {
            format_converter: Box::new(FormatConverterImpl::new()),
        }
    }

    fn initialize_empty_matrix(&self, grid_size: usize) -> Vec<Vec<Cell>> {
        let mut matrix: Vec<Vec<Cell>> = Vec::<Vec<Cell>>::with_capacity(grid_size);
        for i in 0..grid_size {
            let mut line: Vec<Cell> = Vec::<Cell>::with_capacity(grid_size as usize);
            for j in 0..grid_size {
                line.push(Cell::new(
                    DefaultProps::EMPTY_VALUE,
                    false,
                    (i as u8, j as u8),
                ));
            }
            matrix.push(line);
        }
        matrix
    }

    fn step_cursor(
        &self,
        pos: (u8, u8),
        direction: CursorDirection,
        grid_size: u8,
    ) -> Result<(u8, u8), ()> {
        let x: u8 = pos.0;
        let y: u8 = pos.1;
        match direction {
            CursorDirection::Forward => {
                if x == grid_size - 1 && y == grid_size - 1 {
                    return Err(());
                }
                if x == grid_size - 1 {
                    return Ok((0, y + 1));
                }
                Ok((x + 1, y))
            }
            CursorDirection::Back => {
                if x == 0 && y == 0 {
                    return Err(());
                }
                if x == 0 {
                    return Ok((grid_size - 1, y - 1));
                }
                Ok((x - 1, y))
            }
        }
    }
}

impl PuzzleEditor for PuzzleEditorImpl {
    fn create(&self) -> Puzzle {
        let grid_size: u8 = DefaultProps::GRID_SIZE;
        let matrix: Vec<Vec<Cell>> = self.initialize_empty_matrix(grid_size as usize);

        let (mut x_cursor, mut y_cursor) = (0_u8, 0_u8);
        loop {
            let cell: &Cell = &matrix[x_cursor as usize][y_cursor as usize];
            let s = self
                    .format_converter
                    .matrix_to_ss(&matrix, Some(cell.get_position()));
                println!("{}", s);
            
            match self.step_cursor((x_cursor, y_cursor), CursorDirection::Forward, grid_size) {
                Ok(pos) => (x_cursor, y_cursor) = pos,
                Err(()) => break,
            }
        }

        let puzzle: Puzzle = Puzzle::new(grid_size, matrix.clone());
        puzzle
    }
}
