pub fn ascii2u8(ascii: u8) -> u8 {
    ascii - b'0'
}

#[derive(Debug)]
pub(crate) struct Item {
    pub id: String,
    pub name: String,
    pub proteins: f32,
    pub carbohydrates: f32,
    pub total_calories: f32,
    pub total_fats: f32,
}
