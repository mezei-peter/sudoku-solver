use std::fs;

use crate::model::puzzle::Puzzle;

use super::puzzle_parser::PuzzleParser;

pub trait ArgsService {
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl {
    puzzle_parser: Box<dyn PuzzleParser>,
    default_grid_size: u8,
}

impl ArgsServiceImpl {
    pub fn new(puzzle_parser: Box<dyn PuzzleParser>, default_grid_size: u8) -> ArgsServiceImpl {
        ArgsServiceImpl {
            puzzle_parser,
            default_grid_size,
        }
    }

    fn handle_file_arg(&self, file_path: &String) {
        let content =
            fs::read_to_string(file_path).expect(&format!("{} - Invalid file path", file_path));
        if self.invalidate_file(&file_path, &content) {
            return;
        }
        let puzzles: Vec<Puzzle> = self.parse_puzzle_file(&content);
    }

    fn parse_puzzle_file(&self, content: &String) -> Vec<Puzzle> {
        println!("Parsing file...");
        let mut puzzles: Vec<Puzzle> = Vec::new();
        let lines = content.lines();
        for line in lines.into_iter() {
            let puzzle: Puzzle = self.parse_puzzle_line(&line);
            puzzles.push(puzzle);
        }
        println!("Parse successful.");
        puzzles
    }

    fn parse_puzzle_line(&self, line: &str) -> Puzzle {
        let mut puzzle_matrix: Vec<Vec<char>> = Vec::new();
        for _i in 0..self.default_grid_size {
            puzzle_matrix.push(Vec::<char>::new());
        }

        let mut position: u16 = 0;
        let mut row: u8 = 0;
        while position < line.chars().count() as u16 {
            let character: char = line.chars().nth(position as usize).unwrap();
            puzzle_matrix[row as usize].push(character);

            position += 1;
            if position % self.default_grid_size as u16 == 0 {
                row += 1;
            }
        }

        Puzzle::new(self.default_grid_size, puzzle_matrix)
    }

    fn invalidate_file(&self, file_path: &String, content: &String) -> bool {
        if !file_path.ends_with(".sdm") {
            println!("Invalid file format. Acceptable: .sdm");
            return true;
        }
        if content.lines().count() == 1 {
            if self.default_grid_size.pow(2) as usize != content.chars().count() {
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
