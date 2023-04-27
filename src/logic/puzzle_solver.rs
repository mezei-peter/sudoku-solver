use crate::model::{cell::Cell, puzzle::Puzzle};

use super::format_converter::FormatConverter;

pub trait PuzzleSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle;
    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle>;
}

pub struct SudokuSolver {
    format_converter: Box<dyn FormatConverter>,
}

impl SudokuSolver {
    pub fn new(format_converter: Box<dyn FormatConverter>) -> SudokuSolver {
        SudokuSolver { format_converter }
    }

    fn brute_force_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        let mut result_puzzle: Puzzle = puzzle.clone();
        let bound: u8 = puzzle.get_grid_size();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut is_forward: bool = true;

        loop {
            let cell: &Cell = match result_puzzle.get_matrix_cell(x, y) {
                Some(cell) => cell,
                None => break,
            };

            if cell.is_prescribed() {
                let (should_break, new_x, new_y, new_is_forward) =
                    self.identify_prescribed_results(is_forward, x, y, bound);
                if should_break {
                    break;
                }
                (x, y, is_forward) = (new_x, new_y, new_is_forward);
                continue;
            }

            let mut new_cell = cell.clone();
            if self.experiment_valid_cell_value(&mut new_cell, &result_puzzle) {
                result_puzzle
                    .replace_cell_value_at_position(new_cell.get_position(), new_cell.get_value());
                match self.next_position(x, y, bound) {
                    Ok((x_new, y_new)) => {
                        (x, y) = (x_new, y_new);
                        is_forward = true;
                    }
                    Err(()) => break,
                }
            } else {
                result_puzzle.replace_cell_value_at_position(new_cell.get_position(), '0');
                match self.previous_position(x, y, bound) {
                    Ok((x_new, y_new)) => {
                        (x, y) = (x_new, y_new);
                        is_forward = false;
                    }
                    Err(()) => break,
                }
            }
        }
        result_puzzle
    }

    fn next_position(&self, x: usize, y: usize, bound: u8) -> Result<(usize, usize), ()> {
        if x as u8 == bound - 1 && y as u8 == bound - 1 {
            return Err(());
        }
        if x as u8 == bound - 1 {
            return Ok((0, y + 1));
        }
        Ok((x + 1, y))
    }

    fn previous_position(&self, x: usize, y: usize, bound: u8) -> Result<(usize, usize), ()> {
        if x == 0 && y == 0 {
            return Err(());
        }
        if x == 0 {
            return Ok((bound as usize - 1, y - 1));
        }
        Ok((x - 1, y))
    }

    fn experiment_valid_cell_value(&self, cell: &mut Cell, puzzle: &Puzzle) -> bool {
        loop {
            let increment_res = cell.increment_value(puzzle.get_grid_size());
            if increment_res.is_err() {
                return false;
            }
            if puzzle.valid_value_at_position(increment_res.unwrap(), cell.get_position()) {
                return true;
            }
        }
    }

    fn identify_prescribed_results(
        &self,
        is_forward: bool,
        x: usize,
        y: usize,
        bound: u8,
    ) -> (bool, usize, usize, bool) {
        if !is_forward && x == 0 && y == 0 {
            return (true, x, y, is_forward);
        }
        if is_forward {
            match self.next_position(x, y, bound) {
                Ok((x_new, y_new)) => {
                    return (false, x_new, y_new, is_forward);
                }
                Err(()) => return (true, x, y, is_forward),
            };
        } else {
            match self.previous_position(x, y, bound) {
                Ok((x_new, y_new)) => {
                    return (false, x_new, y_new, is_forward);
                }
                Err(()) => return (true, x, y, is_forward),
            }
        }
    }
}

impl PuzzleSolver for SudokuSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        let result: Puzzle = self.brute_force_puzzle(&puzzle);
        result
    }

    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle> {
        let mut count: usize = 1;
        puzzles
            .iter()
            .map(|puzzle: &Puzzle| {
                println!(">>>Puzzle {}<<<", count);
                count += 1;
                println!("Solving puzzle FROM: ");
                println!("{}", self.format_converter.puzzle_to_ss(puzzle));
                println!("TO...: ");
                let solution = self.solve_puzzle(&puzzle);
                println!("{}", self.format_converter.puzzle_to_ss(&solution));
                solution
            })
            .collect()
    }
}
