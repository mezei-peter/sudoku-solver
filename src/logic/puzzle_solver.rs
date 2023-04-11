use crate::model::puzzle::Puzzle;

pub trait SudokuSolver {
    fn solve_puzzle(&self, puzzle: Puzzle) -> Puzzle;
}