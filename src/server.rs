use crate::utils::Item;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use tracing::{error, info, instrument, warn};

use crate::repository::{delete, insert, read, update};

#[instrument]
pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:60000").expect("Failed to bind server");

    let (stream, _) = listener.accept().expect("Failed to accept connection");

    handle_client(stream);
}

pub fn send_response(mut stream: TcpStream, response_payload: String) {
    stream
        .write_all(response_payload.as_bytes())
        .expect("Failed to send response");
    stream.flush().expect("Failed to flush stream");
}

#[instrument]
fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    info!("Client disconnected gracefully.");
                    break;
                }
                let payload = String::from_utf8_lossy(&buffer[..bytes_read]);
                info!("Client: Message received: {}", payload);
                let status_code =
                    handle_operation(payload.as_ref()).expect("Failed to handle operation");
                send_response(stream.try_clone().unwrap(), status_code);
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                error!("Client: Error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn handle_operation(payload: &str) -> Result<String, Error> {
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

    let (status_code, item) = match operation {
        'C' => {
            insert(&create_item_from(id, parts)).expect("Failed to insert item");
            (200, None)
        }
        'U' => {
            let item = create_item_from(id, parts);
            update(&item).expect("Failed to update item");
            (200, None)
        }
        'D' => {
            let status_code = delete(id.as_str()).expect("Failed to delete item");
            (status_code, None)
        }
        'R' => {
            let read_item = read(id.as_str()).expect("Failed to read item");
            (200, read_item.into_iter().next())
        }
        _ => return Err(Error::new(ErrorKind::Other, "Unknown operation")),
    };

    Ok(build_response_payload(operation, status_code, item))
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

fn build_response_payload(operation: char, status_code: u32, item: Option<Item>) -> String {
    match item {
        Some(item) => format!(
            "^{}|{}|{}|{}|{}|{}|{}|{}$",
            operation, status_code, item.id, item.name, item.total_calories, item.carbohydrates,
            item.proteins, item.total_fats
        ),
        None => format!("^{}|{}$", operation, status_code)
    }
}
