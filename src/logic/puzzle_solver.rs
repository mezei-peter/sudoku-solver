use crate::model::{
    cell::{self, Cell},
    default_puzzle_properties::DefaultProps,
    puzzle::Puzzle,
};

pub trait PuzzleSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle;
    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle>;
}

pub struct SudokuSolver;

impl SudokuSolver {
    pub fn new() -> SudokuSolver {
        SudokuSolver {}
    }

    //todo: WORK IN PROGRESS
    fn brute_force_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        let mut result_puzzle: Puzzle = puzzle.clone();
        let bound: u8 = puzzle.get_grid_size();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut is_forward: bool = true;

        loop {
            let cell_res = result_puzzle.get_matrix_cell(x, y);
            if cell_res.is_none() {
                break;
            }

            let cell: &Cell = cell_res.unwrap();
            if cell.is_prescribed() {
                if !is_forward {
                    break;
                }
                let next_res = self.next_position(x, y, bound);
                if next_res.is_err() {
                    break;
                }
                (x, y) = next_res.unwrap();
                is_forward = true;
                continue;
            }

            let mut new_cell = cell.clone();
            if self.experiment_valid_cell_value(&mut new_cell, &result_puzzle) {
                result_puzzle.replace_cell_value_at_position(new_cell.get_position(), new_cell.get_value());
                let next_res = self.next_position(x, y, bound);
                if next_res.is_err() {
                    break;
                }
                (x, y) = next_res.unwrap();
                is_forward = true;
            } else {
                let prev_res = self.prevoius_position(x, y, bound);
                if prev_res.is_err() {
                    break;
                }
                (x, y) = prev_res.unwrap();
                is_forward = false;
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

    fn prevoius_position(&self, x: usize, y: usize, bound: u8) -> Result<(usize, usize), ()> {
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
            let increment_res = cell.increment_value();
            if increment_res.is_err() {
                return false;
            }
            if puzzle.valid_value_at_position(increment_res.unwrap(), cell.get_position()) {
                return true;
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
        puzzles
            .iter()
            .map(|puzzle: &Puzzle| self.solve_puzzle(&puzzle))
            .collect()
    }
}
