use std::env;

use logic::{
    arguments_service::ArgsService,
    puzzle_parser::{PuzzleParser, PuzzleParserImpl},
};

use crate::{
    logic::{
        arguments_service::ArgsServiceImpl,
        puzzle_solver::{PuzzleSolver, SudokuSolver},
    },
};

mod logic {
    pub mod arguments_service;
    pub mod puzzle_parser;
    pub mod puzzle_solver;
}
mod model {
    pub mod cell;
    pub mod default_puzzle_properties;
    pub mod puzzle;
}
mod ui {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle_parser: Box<dyn PuzzleParser> = Box::new(PuzzleParserImpl::new());
    let puzzle_solver: Box<dyn PuzzleSolver> = Box::new(SudokuSolver::new());
    let arg_service: Box<dyn ArgsService> =
        Box::new(ArgsServiceImpl::new(puzzle_parser, puzzle_solver));
    arg_service.process(&args);
}
