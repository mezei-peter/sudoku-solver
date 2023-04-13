#[derive(Debug)]
pub struct Cell {
    value: char,
    prescribed: bool,
}

impl Cell {
    pub fn new(value: char, prescribed: bool) -> Cell {
        Cell { value, prescribed }
    }

    pub fn is_empty(&self) -> bool {
        self.value == '0'
    }
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell {value: self.value.clone(), prescribed: self.prescribed.clone()}
    }
}
