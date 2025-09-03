use crate::repository::{delete, insert, read, update};
use crate::payload;
use crate::utils::{get_item_from_string, get_id};

use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use tracing::{error, info, instrument, warn};

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
                let response_payload =
                    handle_operation(payload.as_ref()).expect("Failed to handle operation");
                send_response(stream.try_clone().unwrap(), response_payload);
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
    let id = get_id(&mut parts);

    let (status_code, item) = match operation {
        'C' => {
            insert(&get_item_from_string(id, parts)).expect("Failed to insert item");
            (200, None)
        }
        'U' => {
            let item = get_item_from_string(id, parts);
            let status_code = update(&item).expect("Failed to update item");
            (status_code, None)
        }
        'D' => {
            let status_code = delete(id.as_str()).expect("Failed to delete item");
            (status_code, None)
        }
        'R' => {
            let read_item = read(id.as_str()).expect("Failed to read item");
            if let Some(item) = read_item.into_iter().next() {
                (200, Some(item))
            } else {
                (404, None)
            }
        }
        _ => return Err(Error::new(ErrorKind::Other, "Unknown operation")),
    };

    Ok(payload::response(operation, status_code, item))
}