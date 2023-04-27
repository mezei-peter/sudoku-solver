use std::{
    fs,
    io::{stdin, Read},
};

use crate::model::{cell::Cell, default_puzzle_properties::DefaultProps, puzzle::Puzzle};

use super::{puzzle_parser::PuzzleParser, puzzle_solver::PuzzleSolver, puzzle_editor::{PuzzleEditor, self}};

pub trait ArgsService {
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl {
    puzzle_editor: Box<dyn PuzzleEditor>,
    puzzle_parser: Box<dyn PuzzleParser>,
    puzzle_solver: Box<dyn PuzzleSolver>,
}

impl ArgsServiceImpl {
    pub fn new(
        puzzle_editor: Box<dyn PuzzleEditor>,
        puzzle_parser: Box<dyn PuzzleParser>,
        puzzle_solver: Box<dyn PuzzleSolver>,
    ) -> ArgsServiceImpl {
        ArgsServiceImpl {
            puzzle_editor,
            puzzle_parser,
            puzzle_solver,
        }
    }

    fn print_help(&self) {
        println!("To use the program, try running it with one of the flags below: \n");
        println!("  -f <file-path>, --file <file-path> : Specify an input sdm file by entering its file path.\n");
        println!("  -i, --interactive : Input a puzzle in an interactive way, answering the program's questions.\n");
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

    fn prompt_user_defined_puzzles(&self) {
        let mut puzzles: Vec<Puzzle> = Vec::<Puzzle>::new();
        loop {
            println!("\nWould you like to create a new puzzle? (y/n)");
            let mut s: String = String::new();
            stdin().read_line(&mut s).expect("Invalid string.");
            s.truncate(1);
            s = s.to_lowercase();

            if s == "y" {
                let puzzle: Puzzle = self.puzzle_editor.create();
                puzzles.push(puzzle);
            } else if s == "n" {
                break;
            } else {
                println!("Invalid input: {}", &s);
            }
        }
    }
}

impl ArgsService for ArgsServiceImpl {
    fn process(&self, args: &Vec<String>) {
        let mut has_valid_arg = false;
        for i in 1..args.len() {
            if args[i] == "-f" || args[i] == "--file" {
                has_valid_arg = true;
                self.handle_file_arg(&args[i + 1]);
            } else if args[i] == "-i" || args[i] == "--interactive" {
                has_valid_arg = true;
                self.prompt_user_defined_puzzles();
            }
        }
        if !has_valid_arg {
            self.print_help();
        }
    }
}
