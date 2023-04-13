use std::fs;

use crate::model::{puzzle::{Puzzle, self}, default_puzzle_properties::DefaultProps};

use super::{puzzle_parser::PuzzleParser, puzzle_solver::{PuzzleSolver, self}};

pub trait ArgsService {
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl {
    puzzle_parser: Box<dyn PuzzleParser>,
    puzzle_solver: Box<dyn PuzzleSolver>,
}

impl ArgsServiceImpl {
    pub fn new(
        puzzle_parser: Box<dyn PuzzleParser>,
        puzzle_solver: Box<dyn PuzzleSolver>,
    ) -> ArgsServiceImpl {
        ArgsServiceImpl {
            puzzle_parser,
            puzzle_solver,
        }
    }

    fn handle_file_arg(&self, file_path: &String) {
        let content =
            fs::read_to_string(file_path).expect(&format!("{} - Invalid file path", file_path));
        if self.invalidate_file(&file_path, &content) {
            return;
        }
        let puzzles: Vec<Puzzle> = self.puzzle_parser.parse_puzzle_file(&content);
        let solved_puzzles: Vec<Puzzle> = self.puzzle_solver.solve_all_puzzles(&puzzles);
        dbg!("{:?}", solved_puzzles);
    }

    fn invalidate_file(&self, file_path: &String, content: &String) -> bool {
        if !file_path.ends_with(".sdm") {
            println!("Invalid file format. Acceptable: .sdm");
            return true;
        }
        if content.lines().count() == 1 {
            if DefaultProps::GRID_SIZE.pow(2) as usize != content.chars().count() {
                println!("Corrupted file data: length of lines are wrong.");
                return true;
            }
        }

        println!("File validated.");
        false
    }
}

impl ArgsService for ArgsServiceImpl {
    fn process(&self, args: &Vec<String>) {
        for i in 1..args.len() {
            if args[i].starts_with("--") {
                if args[i].eq("--file") {
                    self.handle_file_arg(&args[i + 1]);
                }
                continue;
            }

            if args[i].starts_with("-") {
                if args[i].ends_with("f") {
                    self.handle_file_arg(&args[i + 1]);
                }
                continue;
            }
        }
    }
}
