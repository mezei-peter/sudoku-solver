#[derive(Debug)]
pub struct Cell {
    value: char,
    prescribed: bool,
}

impl Cell {
    pub fn new(value: char, prescribed: bool) -> Cell {
        Cell { value, prescribed }
    }
}
