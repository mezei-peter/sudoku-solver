use crate::model::puzzle::Puzzle;

pub trait PuzzleEditor {
    fn create(&self) -> Puzzle;
}

pub struct PuzzleEditorImpl;

impl PuzzleEditorImpl {
    pub fn new() -> PuzzleEditorImpl {
        PuzzleEditorImpl
    }
}

impl PuzzleEditor for PuzzleEditorImpl {
    fn create(&self) -> Puzzle {
        todo!()
    }
}
