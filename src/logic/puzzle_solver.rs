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

        let mut is_direction_forward = true;
        loop {
            let cell_res = if is_direction_forward {
                result_puzzle.next_cell()
            } else {
                if !is_direction_forward && result_puzzle.is_initial_pos() {
                    break;
                }
                result_puzzle.previous_cell()
            };

            if cell_res.is_err() {
                break;
            }
            let cell: &mut Cell = cell_res.unwrap();

            loop {
                let increment_res = cell.increment_value();
                if increment_res.is_err() {
                    is_direction_forward = false;
                    cell.reset_value();
                    break;
                } else {
                    is_direction_forward = true;
                }
                if puzzle.accept_cell(cell) {
                    break;
                }
            }
        }

        result_puzzle
    }
}

impl PuzzleSolver for SudokuSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        self.brute_force_puzzle(&puzzle)
    }

    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle> {
        puzzles
            .iter()
            .map(|puzzle: &Puzzle| self.solve_puzzle(&puzzle))
            .collect()
    }
}
