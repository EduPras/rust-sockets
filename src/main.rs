use tracing_subscriber;
use crate::utils::Item;

mod server;
mod utils;
mod repository;

mod client;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    // server::listen()?;
    let item = Item{
        id: "teste1".to_string(),
        name: "Suco de banana".to_string(),
        proteins: 100.0,
        carbohydrates: 901.0,
        total_calories: 133.0,
        total_fats: 133.0
    };
    let items = repository::read("teste1".to_string()).unwrap();
    for item in items {
        println!("{:?}", item);
    }
    let item = Item{
        id: "teste1".to_string(),
        name: "Suco de banana".to_string(),
        proteins: 155.0,
        carbohydrates: 901.0,
        total_calories: 133.0,
        total_fats: 133.0
    };
    let result = repository::update(item);
    let items = repository::read("teste1".to_string()).unwrap();
    for item in items {
        println!("{:?}", item);
    }

    let result = repository::delete("teste1".to_string());

    client::start()?;
    Ok(())
}
