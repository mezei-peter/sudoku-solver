use crate::model::{puzzle::Puzzle, cell::Cell};

pub trait PuzzleSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle;
    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle>;
}

pub struct SudokuSolver;

impl SudokuSolver {
    pub fn new() -> SudokuSolver {
        SudokuSolver {}
    }
}

impl PuzzleSolver for SudokuSolver {
    fn solve_puzzle(&self, puzzle: &Puzzle) -> Puzzle {
        let puzzle: Puzzle = puzzle.clone();

        puzzle
    }

    fn solve_all_puzzles(&self, puzzles: &Vec<Puzzle>) -> Vec<Puzzle> {
        puzzles
            .iter()
            .map(|puzzle: &Puzzle| self.solve_puzzle(&puzzle))
            .collect()
    }
}
