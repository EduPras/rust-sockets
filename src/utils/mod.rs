pub fn ascii2u8(ascii: u8) -> u8 {
    ascii - b'0'
}

#[derive(Debug)]
pub struct Column {
    pub col_type: u8, // 0 = integer, 1 = string, 2 = float
    pub data: Vec<u8>,
}
pub struct Item {
    pub columns: Vec<Column>,
}

