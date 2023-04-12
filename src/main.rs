use std::env;

use logic::{arguments_service::ArgsService, puzzle_parser::{PuzzleParser, PuzzleParserImpl}};

use crate::logic::arguments_service::ArgsServiceImpl;

mod logic {
    pub mod arguments_service;
    pub mod puzzle_parser;
    pub mod puzzle_solver;
}
mod model {
    pub mod puzzle;
}
mod ui {}

fn main() {
    let args: Vec<String> = env::args().collect();
    const DEFAULT_GRID_SIZE: u8 = 9;
    let puzzle_parser: Box<dyn PuzzleParser> = Box::new(PuzzleParserImpl::new(DEFAULT_GRID_SIZE));
    let arg_service: Box<dyn ArgsService> = Box::new(ArgsServiceImpl::new(puzzle_parser, DEFAULT_GRID_SIZE));
    arg_service.process(&args);
}
