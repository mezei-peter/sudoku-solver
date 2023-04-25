use super::default_puzzle_properties::DefaultProps;

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

    pub fn is_prescribed(&self) -> bool {
        self.prescribed
    }

    pub fn increment_value(&mut self) -> Result<char, ()> {
        let original: u32 = self.value.to_digit(10).unwrap();
        if original + 1 > DefaultProps::GRID_SIZE as u32 {
            return Err(());
        }

        let new_value: char = char::from_u32(original + 1).unwrap();
        self.value = char::from_u32(original + 1).unwrap();
        Ok(new_value)
    }
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell {value: self.value.clone(), prescribed: self.prescribed.clone()}
    }
}
