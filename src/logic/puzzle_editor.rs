use crate::model::{default_puzzle_properties::DefaultProps, puzzle::Puzzle, cell::Cell};

use super::format_converter::{FormatConverter, self, FormatConverterImpl};

pub trait PuzzleEditor {
    fn create(&self) -> Puzzle;
}

pub struct PuzzleEditorImpl {
    format_converter: Box<dyn FormatConverter>,
}

impl PuzzleEditorImpl {
    pub fn new() -> PuzzleEditorImpl {
        PuzzleEditorImpl{format_converter: Box::new(FormatConverterImpl::new())}
    }

    fn initialize_empty_matrix(&self, grid_size: usize) -> Vec<Vec<Cell>> {
        let mut matrix: Vec<Vec<Cell>> = Vec::<Vec<Cell>>::with_capacity(grid_size);
        for i in 0..grid_size {
            let mut line: Vec<Cell> = Vec::<Cell>::with_capacity(grid_size as usize);
            for j in 0..grid_size {
                line.push(Cell::new(DefaultProps::EMPTY_VALUE, false, (i as u8, j as u8)));
            }
            matrix.push(line);
        }
        matrix
    }
}

impl PuzzleEditor for PuzzleEditorImpl {
    fn create(&self) -> Puzzle {
        let grid_size: u8 = DefaultProps::GRID_SIZE;
        let empty_matrix: Vec<Vec<Cell>> = self.initialize_empty_matrix(grid_size as usize);
        let puzzle: Puzzle = Puzzle::new(grid_size, empty_matrix);

        let s: String = self.format_converter.puzzle_to_ss(&puzzle);
        println!("{}", s);
        puzzle
    }
}
