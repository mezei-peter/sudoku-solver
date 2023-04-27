pub struct DefaultProps;

impl DefaultProps {
    pub const GRID_SIZE: u8 = 9;
    pub const EMPTY_VALUE: char = '0';
    pub const CURSOR_VALUE: char = 'X';
    pub const EMPTY_SS_VALUE: char = '.';
    pub const VALID_INPUTS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
}