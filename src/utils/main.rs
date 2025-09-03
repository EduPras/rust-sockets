#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub proteins: f32,
    pub carbohydrates: f32,
    pub total_calories: f32,
    pub total_fats: f32,
}

pub fn get_id(parts: &mut std::str::Split<'_, char>) -> String {
    parts
        .next()
        .expect("Missing product hash")
        .parse()
        .expect("Fail to convert product_hash")
}

pub fn get_item_from_string(id: String, mut parts: std::str::Split<'_, char>) -> Item {
    let product_name: String = parts
        .next()
        .expect("Missing product name")
        .parse()
        .expect("Fail to convert product_name");

    let calories: f32 = parts
        .next()
        .expect("Missing calories")
        .parse()
        .expect("Fail to convert calories");

    let carbo: f32 = parts
        .next()
        .expect("Missing carbo")
        .parse()
        .expect("Fail to convert carbo");

    let fat: f32 = parts
        .next()
        .expect("Missing fat")
        .parse()
        .expect("Fail to convert fat");

    let protein: f32 = parts
        .next()
        .expect("Missing protein")
        .parse()
        .expect("Fail to convert protein");

    Item {
        id,
        name: product_name,
        proteins: protein,
        carbohydrates: carbo,
        total_calories: calories,
        total_fats: fat,
    }
}