use crate::model::puzzle::Puzzle;

pub trait PuzzleSolver {
    fn solve_puzzle(&self, puzzle: Puzzle) -> Puzzle;
    fn solve_all_puzzles(&self, puzzles: Vec<Puzzle>) -> Vec<Puzzle>;
}

pub struct SudokuSolver;

impl SudokuSolver {

}

impl PuzzleSolver for SudokuSolver {
    fn solve_puzzle(&self, puzzle: Puzzle) -> Puzzle {
        todo!()
    }

    fn solve_all_puzzles(&self, puzzles: Vec<Puzzle>) -> Vec<Puzzle> {
        todo!()
    }
}