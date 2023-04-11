use std::env;

use logic::arguments_service::ArgsService;

use crate::logic::arguments_service::ArgsServiceImpl;

mod logic {
    pub mod arguments_service;
    pub mod puzzle_solver;
}
mod model {
    pub mod puzzle;
}
mod ui {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_service = ArgsServiceImpl::new();
    arg_service.process(&args);
}
