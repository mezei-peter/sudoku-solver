use crate::model::{cell::Cell, default_puzzle_properties::DefaultProps, puzzle::Puzzle};

pub trait PuzzleParser {
    fn parse_puzzle_file(&self, content: &String) -> Vec<Puzzle>;
    fn parse_puzzle(&self, data: &str) -> Puzzle;
}

pub struct PuzzleParserImpl;

impl PuzzleParserImpl {
    pub fn new() -> PuzzleParserImpl {
        PuzzleParserImpl {}
    }
}

impl PuzzleParser for PuzzleParserImpl {
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
        let mut puzzle_matrix: Vec<Vec<Cell>> = Vec::new();
        for _i in 0..DefaultProps::GRID_SIZE {
            puzzle_matrix.push(Vec::<Cell>::new());
        }

        let mut position: u16 = 0;
        let mut row: u8 = 0;
        let mut col: u8 = 0;
        while position < line.chars().count() as u16 {
            let ch: char = line.chars().nth(position as usize).unwrap();
            let cell: Cell = Cell::new(
                ch,
                if ch == '0' { false } else { true },
                (row, col),
            );

            puzzle_matrix[row as usize].push(cell);

            position += 1;
            col += 1;
            if position % DefaultProps::GRID_SIZE as u16 == 0 {
                row += 1;
                col = 0;
            }
        }


        Puzzle::new(DefaultProps::GRID_SIZE, puzzle_matrix)
    }
}
