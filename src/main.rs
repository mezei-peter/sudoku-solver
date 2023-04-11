use std::env;

mod logic {
    pub mod puzzle_solver;
}
mod model {
    pub mod puzzle;
}
mod ui {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let first: &String = &args[1];

    println!("{}", first);
}
