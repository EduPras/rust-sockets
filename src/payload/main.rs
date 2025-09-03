use md5::{Digest, Md5};
use crate::utils::Item;
use crate::utils::read::{read_operation, read_float, read_product_name};


pub fn response(operation: char, status_code: u32, item: Option<Item>) -> String {
    match item {
        Some(item) => format!(
            "^{}|{}|{}|{}|{}|{}|{}|{}$",
            operation,
            status_code,
            item.id,
            item.name,
            item.total_calories,
            item.carbohydrates,
            item.proteins,
            item.total_fats
        ),
        None => format!("^{}|{}$", operation, status_code),
    }
}

pub fn request() -> String {
    let operation = read_operation();
    let mut product_name = read_product_name();
    let id = get_id_md5(&mut product_name);

    match operation {
        'C' | 'U' => {
            let calories = read_float("calorias");
            let carbo = read_float("carbo");
            let protein = read_float("proteina");
            let fats = read_float("gordura");
            format!(
                "^{}|{}|{}|{}|{}|{}|{}$",
                operation, id, product_name, calories, carbo, protein, fats
            )
        }
        'R' | 'D' => {
            format!("^{}|{}|{}$", operation, id, product_name)
        }
        _ => todo!(),
    }
}

pub fn get_id_md5(product_name: &mut String) -> String {
    let mut hasher = Md5::new();
    hasher.update(product_name.as_bytes());
    let hash = hasher.finalize();
    let mut buffer = [0u8; 32];
    let encoded_bytes =
        base16ct::lower::encode(&hash, &mut buffer).expect("Failed to encode hash as base16");
    std::str::from_utf8(encoded_bytes)
        .expect("Encoded bytes were not valid UTF-8")
        .to_string()
}
