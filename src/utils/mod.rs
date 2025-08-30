pub fn ascii2u8(ascii: u8) -> u8 {
    ascii - b'0'
}

#[derive(Debug)]
 struct Item {
    pub id: Option<String>,
    pub name: Option<String>,
    pub proteins: Option<f32>,
    pub carbohydrates: Option<f32>,
    pub total_calories: Option<f32>,
    pub total_fats: Option<f32>,
}

impl Item {
    fn new(id: String) -> Item {
        Item {
            id: Some(id),
            name: None,
            proteins: None,
            carbohydrates: None,
            total_calories: None,
            total_fats: None,
        }
    }
}
