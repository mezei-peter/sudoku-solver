use crate::model::{cell::Cell, puzzle::Puzzle};

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
    fn brute_force_puzzle(&self, puzzle: &mut Puzzle) {
        loop {
            let next_cell_res = puzzle.next_cell();
            if next_cell_res.is_err() {
                break;
            }

            let mut next_cell: &mut Cell = next_cell_res.unwrap();
            if (next_cell.is_prescribed()) {
                continue;
            }

        }
    }
}

impl PuzzleSolver for SudokuSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        let mut puzzle: Puzzle = puzzle.clone();
        self.brute_force_puzzle(&mut puzzle);
        puzzle
    }

    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle> {
        puzzles
            .iter()
            .map(|puzzle: &Puzzle| self.solve_puzzle(&puzzle))
            .collect()
    }
}
