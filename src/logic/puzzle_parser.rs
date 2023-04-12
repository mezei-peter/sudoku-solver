use crate::model::puzzle::Puzzle;

pub trait PuzzleParser {
    fn parse_puzzle_file(&self, content: &String) -> Vec<Puzzle>;
    fn parse_puzzle(&self, data: &str) -> Puzzle;
}

pub struct PuzzleParserImpl {
    default_grid_size: u8,
}

impl PuzzleParser for PuzzleParserImpl {
    fn new(default_grid_size: u8) -> PuzzleParserImpl {
        PuzzleParserImpl { default_grid_size }
    }

    fn parse_puzzle_file(&self, content: &String) -> Vec<Puzzle> {
        println!("Parsing file...");
        let mut puzzles: Vec<Puzzle> = Vec::new();
        let lines = content.lines();
        for line in lines.into_iter() {
            let puzzle: Puzzle = self.parse_puzzle(&line);
            puzzles.push(puzzle);
        }
        println!("Parse successful.");
        puzzles
    }

    fn parse_puzzle(&self, line: &str) -> Puzzle {
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
}
