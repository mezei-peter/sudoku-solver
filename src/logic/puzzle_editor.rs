use std::io::stdin;

use crate::{
    model::{cell::Cell, default_puzzle_properties::DefaultProps, puzzle::Puzzle},
    ui::input_reader::{InputReader, InputReaderImpl},
};

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
        let mut matrix: Vec<Vec<Cell>> = self.initialize_empty_matrix(grid_size as usize);
        let mut puzzle: Puzzle = Puzzle::new(grid_size, matrix);

        let (mut x_cursor, mut y_cursor) = (0_u8, 0_u8);
        loop {
            let cell: &Cell = puzzle
                .get_matrix_cell(x_cursor as usize, y_cursor as usize)
                .unwrap();
            let mat_str = self
                .format_converter
                .puzzle_to_ss(&puzzle, Some(cell.get_position()));
            println!("{}", mat_str);
            println!("The cursor's position is marked by X. You can: ");
            println!("  - Type 'FINISH' to submit your puzzle.");
            println!("  - Press the Enter key to move forward.");
            println!("  - Type 'b' to move the cursor back.");
            println!("  - Type a value like '3' to insert a value.");
            let input = InputReaderImpl::read_line();
            if input == "b" {
                println!("BACK");
                match self.step_cursor((x_cursor, y_cursor), CursorDirection::Back, grid_size) {
                    Ok(pos) => {
                        (x_cursor, y_cursor) = pos;
                        continue;
                    }
                    Err(()) => {
                        (x_cursor, y_cursor) = (grid_size - 1, grid_size - 1);
                        continue;
                    }
                }
            } else if input == "FINISH" {
                break;
            } else if input.len() == 0 {
                println!("NEXT");
            } else if input.len() != 1
                || !DefaultProps::VALID_INPUTS.contains(&input.chars().nth(0).unwrap())
            {
                println!("!! ERROR Invalid input: '{}' !!", input);
                continue;
            } else {
                let mut inp_char: char = input.chars().nth(0).unwrap();
                if inp_char == DefaultProps::EMPTY_SS_VALUE {
                    inp_char = DefaultProps::EMPTY_VALUE;
                }
                puzzle.replace_cell_value_at_position((x_cursor, y_cursor), inp_char);

                if inp_char == DefaultProps::EMPTY_VALUE {
                    puzzle.prescribe_cell_at_position((x_cursor, y_cursor), false);
                }
                puzzle.prescribe_cell_at_position((x_cursor, y_cursor), true);
            }

            match self.step_cursor((x_cursor, y_cursor), CursorDirection::Forward, grid_size) {
                Ok(pos) => (x_cursor, y_cursor) = pos,
                Err(()) => (x_cursor, y_cursor) = (0, 0),
            }
        }

        println!("\n!! Puzzle submitted !!\n");
        puzzle
    }
}
