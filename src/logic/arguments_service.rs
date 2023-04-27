use std::fs;

use crate::model::{default_puzzle_properties::DefaultProps, puzzle::Puzzle};

use super::{puzzle_parser::PuzzleParser, puzzle_solver::PuzzleSolver};

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

    fn print_help(&self) {
        println!("To use the program, try running it with one of the flags below: \n");
        println!("  -f <file-path>, --file <file-path> : Specify an input sdm file by entering its file path.\n")
    }

    fn handle_file_arg(&self, file_path: &String) {
        let content =
            fs::read_to_string(file_path).expect(&format!("{} - Invalid file path", file_path));
        if self.invalidate_file(&file_path, &content) {
            return;
        }
        let puzzles: Vec<Puzzle> = self.puzzle_parser.parse_puzzle_file(&content);
        let _solved_puzzles: Vec<Puzzle> = self.puzzle_solver.solve_all_puzzles(&puzzles);
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
        let mut has_valid_arg = false;
        for i in 1..args.len() {
            if args[i] == "-f" || args[i] == "--file" {
                self.handle_file_arg(&args[i + 1]);
                has_valid_arg = true;
            }
        }
        if !has_valid_arg {
            self.print_help();
        }
    }
}
