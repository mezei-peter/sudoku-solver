use std::env;

use logic::{
    arguments_service::ArgsService,
    format_converter::{FormatConverter, FormatConverterImpl},
    puzzle_parser::{PuzzleParser, PuzzleParserImpl},
};

use crate::logic::{
    arguments_service::ArgsServiceImpl,
    puzzle_solver::{PuzzleSolver, SudokuSolver},
};

mod logic {
    pub mod arguments_service;
    pub mod format_converter;
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
    println!("\n>> Sudoku Solver in Rust <<\n");
    let args: Vec<String> = env::args().collect();
    let puzzle_parser: Box<dyn PuzzleParser> = Box::new(PuzzleParserImpl::new());
    let format_converter: Box<dyn FormatConverter> = Box::new(FormatConverterImpl::new());
    let puzzle_solver: Box<dyn PuzzleSolver> = Box::new(SudokuSolver::new(format_converter));
    let arg_service: Box<dyn ArgsService> =
        Box::new(ArgsServiceImpl::new(puzzle_parser, puzzle_solver));
    arg_service.process(&args);
}
