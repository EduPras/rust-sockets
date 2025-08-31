use crate::utils::Item;
use std::io::{Read, Split};
use std::net::{TcpListener, TcpStream};
use std::str::Chars;
use tracing::{Level, debug, error, info, instrument, span, warn};

#[instrument]
pub fn listen() -> std::io::Result<u8> {
    let listener = TcpListener::bind("127.0.0.1:60000")?;

    // accept connections and process them serially
    match listener.accept() {
        Ok((stream, _)) => handle_client(stream),
        Err(e) => error!("Connection failed: {}", e),
    }

    Ok(200)
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let payload = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received payload from client: {payload}");
            handle_operation(payload.as_ref())
        }
        Err(e) => println!("Error reading TcpStream: {e}"),
    }
}

fn handle_operation(payload: &str) {
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
    let id = extract_id_from(&mut parts);

    match operation {
        'C' => {
            let item = create_item_from(id, parts);
        }
        'R' => read_item(id),
        'U' => {
            let item = create_item_from(id, parts);
            update_item(item)
        }
        'D' => delete_item(id),
        _ => warn!("===> Unknown operation"),
    }
}

fn extract_id_from(parts: &mut std::str::Split<'_, char>) -> String {
    parts
        .next()
        .expect("Missing product hash")
        .parse()
        .expect("Fail to convert product_hash")
}

fn create_item_from(id: String, mut parts: std::str::Split<'_, char>) -> Item {
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

// ^C|Suco de Laranja|1000|50|20|5$
// CRUD logic
// Read
fn read_item(id: String) {
    /*
    BYTES
    id: 0..32 bytes (MD5)
     */
    println!("===> DELETE");
}

// Update
fn update_item(item: Item) {
    /*
    BYTES
    id: 0..32 bytes (MD5)
    number of columns: 32..33 byte (u8)
        (for each column / first column example)
        column number: 33..34 bytes (u8)
        column type: 34..35 bytes (u8)
        column data_length: 35..36 bytes (u8 -> string max 256 chars)
        column data: 36..data_length bytes (raw bytes)
    */
    println!("===> UPDATE");
}

// delete
fn delete_item(id: String) {
    /*
    BYTES
    id: 0..32 bytes (MD5)
     */
    println!("===> DELETE");
}
