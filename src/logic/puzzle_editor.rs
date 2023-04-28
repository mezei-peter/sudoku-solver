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

enum InputResult {
    Back((u8, u8)),
    Next((u8, u8)),
    Error,
    ChangeValue((u8, u8)),
    Finish,
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

    fn print_puzzle_with_cursor(&self, puzzle: &Puzzle, cursor_pos: (u8, u8)) {
        let mat_str = self
            .format_converter
            .puzzle_to_ss(&puzzle, Some(cursor_pos));
        println!("{}", mat_str);
        println!("The cursor's position is marked by X. You can: ");
        println!("  - Type 'FINISH' to submit your puzzle.");
        println!("  - Press the Enter key to move forward.");
        println!("  - Type 'b' to move the cursor back.");
        println!("  - Type a value like '3' to insert a value.");
    }

    fn alter_puzzle_cell(&self, puzzle: &mut Puzzle, position: (u8, u8), input: &String) {
        let mut inp_char: char = input.chars().nth(0).unwrap();
        if inp_char == DefaultProps::EMPTY_SS_VALUE || inp_char == DefaultProps::EMPTY_VALUE {
            inp_char = DefaultProps::EMPTY_VALUE;
            puzzle.prescribe_cell_at_position(position, false);
        } else {
            puzzle.prescribe_cell_at_position(position, true);
        }
        puzzle.replace_cell_value_at_position(position, inp_char);
    }

    fn determine_input(&self, input: &str, position: (u8, u8), grid_size: u8) -> InputResult {
        if input == "b" {
            match self.step_cursor(position, CursorDirection::Back, grid_size) {
                Ok(new_pos) => return InputResult::Back(new_pos),
                Err(()) => return InputResult::Back((grid_size - 1, grid_size - 1)),
            }
        } else if input.len() == 0 {
            match self.step_cursor(position, CursorDirection::Forward, grid_size) {
                Ok(new_pos) => return InputResult::Next(new_pos),
                Err(()) => return InputResult::Next((0, 0)),
            }
        } else if input.len() > 1
            || !DefaultProps::VALID_INPUTS.contains(&input.chars().nth(0).unwrap())
        {
            if input == "FINISH" {
                return InputResult::Finish;
            }
            return InputResult::Error;
        } else {
            match self.step_cursor(position, CursorDirection::Forward, grid_size) {
                Ok(new_pos) => return InputResult::ChangeValue(new_pos),
                Err(()) => return InputResult::ChangeValue((0, 0)),
            }
        }
    }
}

impl PuzzleEditor for PuzzleEditorImpl {
    fn create(&self) -> Puzzle {
        let grid_size: u8 = DefaultProps::GRID_SIZE;
        let mut puzzle: Puzzle =
            Puzzle::new(grid_size, self.initialize_empty_matrix(grid_size as usize));
        let (mut x_cursor, mut y_cursor) = (0_u8, 0_u8);
        loop {
            self.print_puzzle_with_cursor(&puzzle, (x_cursor, y_cursor));
            let input: String = InputReaderImpl::read_line();
            match self.determine_input(&input, (x_cursor, y_cursor), grid_size) {
                InputResult::Back(new_pos) => {
                    println!("BACK");
                    (x_cursor, y_cursor) = new_pos;
                }
                InputResult::Next(new_pos) => {
                    println!("NEXT");
                    (x_cursor, y_cursor) = new_pos;
                }
                InputResult::Error => println!("!! ERROR Invalid input: '{}' !!", input),
                InputResult::ChangeValue(new_pos) => {
                    self.alter_puzzle_cell(&mut puzzle, (x_cursor, y_cursor), &input);
                    (x_cursor, y_cursor) = new_pos;
                }
                InputResult::Finish => break,
            }
        }
        println!("\n!! Puzzle submitted !!\n");
        puzzle
    }
}
