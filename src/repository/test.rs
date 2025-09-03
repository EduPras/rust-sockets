use crate::repository::main::{delete, insert, read, update};

const DB_PATH: &str = "database.sqlite3";
#[cfg(test)]
mod test {
    use crate::utils::Item;

    fn setup() {
        let _ = std::fs::remove_file(super::DB_PATH);
    }

    #[test]
    fn test_basic_crud() {
        setup();

        // CREATE
        let id = "unit_test_7";
        let item1 = Item {
            id: id.to_string(),
            name: "Suco de banana".to_string(),
            proteins: 100.0,
            carbohydrates: 901.0,
            total_calories: 133.0,
            total_fats: 133.0,
        };

        assert!(super::insert(&item1).is_ok());
        println!("Created OK");

        // UPDATE
        let item2 = Item {
            id: id.to_string(),
            name: "Suco de banana com morango".to_string(),
            proteins: 100.0,
            carbohydrates: 10.0, // This is the value we will check.
            total_calories: 133.0,
            total_fats: 133.0,
        };

        assert!(super::update(&item2).is_ok());
        println!("Updated OK");

        // READ
        let result = super::read(id);
        assert!(result.is_ok());
        let items = result.expect("Failed to get items");

        assert_eq!(items.len(), 1);
        let first_item = &items[0];

        assert_eq!(first_item.carbohydrates, 10.0f32);
        println!("Read and verified OK");

        // DELETE
        let result = super::delete(id);
        assert!(result.is_ok());
        println!("Deleted OK");

        let final_result = super::read(id);
        assert!(final_result.is_ok());
        assert!(final_result.unwrap().is_empty(), "Item should be deleted");
        println!("Verified delete OK");
    }
}
