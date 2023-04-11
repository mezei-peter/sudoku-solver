use std::env;

use crate::logic::arguments_service::ArgumentsService;

mod logic {
    pub mod puzzle_solver;
    pub mod arguments_service;
}
mod model {
    pub mod puzzle;
}
mod ui {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_service: ArgumentsService = ArgumentsService::new();
    arg_service.process(&args);
}
