use std::borrow::Cow;
use crate::item::Item;

pub fn handle_server_response(payload: Cow<str>) {
    let content = payload
        .strip_prefix("^")
        .expect("Invalid prefix")
        .strip_suffix("$")
        .expect("Invalid suffix");

    let mut parts = content.split('|');

    let operation: char = parts
        .next()
        .expect("Missing operation")
        .parse()
        .expect("Fail to convert operation");

    let status_code: u32 = parts
        .next()
        .expect("Missing status code")
        .parse()
        .expect("Fail to convert status code");

    match operation {
        'C' | 'U' | 'D' => {
            if status_code == 200 {
                println!("Operation {} succeeded with status code {}",
                         operation_name(operation), status_code);
                return;
            }
            println!("Operation {} failed with status code {}",
                     operation_name(operation), status_code);
        }
        'R' => {
            if status_code == 200 {
                println!("Operation {} succeeded with status code {} - Retrieved: {:?}",
                         operation_name(operation), status_code, create_item_from(parts));
                return;
            }
            println!("Operation {} failed with status code {}",
                     operation_name(operation), status_code);
        }
        _ => {}
    }
}
fn operation_name(operation: char) -> String {
    match operation {
        'C' => "create",
        'R' => "read",
        'U' => "update",
        'D' => "delete",
        _ => todo!()
    }.to_string()
}

fn create_item_from(mut parts: std::str::Split<'_, char>) -> Item {
    let id: String = parts
        .next()
        .expect("Missing product hash")
        .parse()
        .expect("Fail to convert product_hash");

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