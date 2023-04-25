use super::default_puzzle_properties::DefaultProps;

#[derive(Debug)]
pub struct Cell {
    value: char,
    prescribed: bool,
    position: (u8, u8),
}

impl Cell {
    pub fn new(value: char, prescribed: bool, position: (u8, u8)) -> Cell {
        Cell {
            value,
            prescribed,
            position,
        }
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

        self.value = char::from_digit(original + 1, 10).unwrap() as char;
        Ok(self.value)
    }

    pub fn get_value(&self) -> char {
        self.value
    }

    pub fn reset_value(&mut self) {
        self.value = '0';
    }
}

impl Clone for Cell {
    fn clone(&self) -> Cell {
        Cell {
            value: self.value.clone(),
            prescribed: self.prescribed.clone(),
            position: self.position.clone(),
        }
    }
}
