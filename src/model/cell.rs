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
        self.value == DefaultProps::EMPTY_VALUE
    }

    pub fn is_prescribed(&self) -> bool {
        self.prescribed
    }

    pub fn increment_value(&mut self, grid_size: u8) -> Result<char, ()> {
        let original: u32 = self.value.to_digit(10).unwrap();
        if original + 1 > grid_size as u32 {
            return Err(());
        }

        self.value = char::from_digit(original + 1, 10).unwrap() as char;
        Ok(self.value)
    }

    pub fn get_value(&self) -> char {
        self.value
    }

    pub fn reset_value(&mut self) {
        self.value = DefaultProps::EMPTY_VALUE;
    }

    pub fn get_pos_x(&self) -> u8 {
        self.position.0
    }

    pub fn get_pos_y(&self) -> u8 {
        self.position.1
    }

    pub fn get_position(&self) -> (u8, u8) {
        self.position
    }

    pub fn set_value(&mut self, value: char) {
        self.value = value;
    }

    pub fn has_value(&self, value: char) -> bool {
        self.value == value
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
