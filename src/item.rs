#[derive(Debug)]
pub(crate) struct Item {
    pub id: String,
    pub name: String,
    pub proteins: f32,
    pub carbohydrates: f32,
    pub total_calories: f32,
    pub total_fats: f32,
}