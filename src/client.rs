use md5::{Digest, Md5};
use std::borrow::Cow;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use rusqlite::params;
use tracing::{error, info};
use crate::utils::Item;

pub fn start() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:60000")?;
    info!("Client: Connected to server.");

    loop {
        let payload = build_payload();

        // Exit loop
        if payload.trim().eq_ignore_ascii_case("exit") {
            info!("Client: Exiting.");
            break;
        }

        stream.write_all(payload.as_bytes())?;
        stream.flush()?;
        info!("Client: Message sent: {}", payload.trim());

        // Response
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    info!("Client: Server closed the connection.");
                    break;
                }

                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Server response: {}", response);
                handle_server_response(response);
            }
            Err(e) => {
                error!("Client: Failed to read from stream: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn handle_server_response(payload: Cow<str>) {
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
                println!("Operation {}, succeeded with status code {}",
                         operation_name(operation), status_code);
                return;
            }
            println!("Operation {}, failed with status code {}",
                     operation_name(operation), status_code);
        }
        'R' => {
            println!("Operation {}, succeeded with status code {} - Retrieved Item: {:?}",
                     operation_name(operation), status_code, create_item_from(parts));
        }
        _ => {}
    }
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

fn operation_name(operation: char) -> String {
    match operation {
        'C' => "create",
        'R' => "read",
        'U' => "update",
        'D' => "delete",
        _ => todo!()
    }.to_string()
}

fn build_payload() -> String {
    let operation = read_operation();
    let mut product_name = read_product_name();
    let id = build_id_from(&mut product_name);

    match operation {
        'C' | 'U' => {
            let calories = read_calories();
            let carbo = read_carbo();
            let protein = read_protein();
            let fats = read_fats();
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

fn build_id_from(product_name: &mut String) -> String {
    let mut hasher = Md5::new();
    hasher.update(product_name.as_bytes()); // Input data as bytes
    let hash = hasher.finalize(); // The result is a [u8; 16] array for MD5
    let mut buffer = [0u8; 32];
    let encoded_bytes =
        base16ct::lower::encode(&hash, &mut buffer).expect("Failed to encode hash as base16");
    std::str::from_utf8(encoded_bytes)
        .expect("Encoded bytes were not valid UTF-8")
        .to_string()
}

fn read_operation() -> char {
    loop {
        print!(
            "Selecione a operação:\n\
            C - create\n\
            R - read\n\
            U - update\n\
            D - delete\n> "
        );
        io::stdout().flush().expect("Falha ao flushar stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        let trimmed_input = input.trim();
        if let Some(char_input) = trimmed_input.chars().next() {
            let upper_char = char_input.to_ascii_uppercase();
            match upper_char {
                'C' | 'R' | 'U' | 'D' => return upper_char,
                _ => println!("Caractere inválido. Tente novamente."),
            }
        } else {
            println!("Entrada vazia. Tente novamente.");
        }
    }
}

fn read_product_name() -> String {
    print!("Informe o nome do produto: ");
    io::stdout().flush().expect("Falha ao flushar stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input
        .trim()
        .replace(|c: char| "^$|".contains(c), "")
        .to_string()
}

fn read_calories() -> f32 {
    loop {
        print!("Informe as calorias: ");
        io::stdout().flush().expect("Falha ao flushar stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número.");
            }
        }
    }
}

fn read_carbo() -> f32 {
    loop {
        print!("Informe o carbo: ");
        io::stdout().flush().expect("Falha ao flushar stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número.");
            }
        }
    }
}

fn read_protein() -> f32 {
    loop {
        print!("Informe a proteina: ");
        io::stdout().flush().expect("Falha ao flushar stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número.");
            }
        }
    }
}

fn read_fats() -> f32 {
    loop {
        print!("Informe a gordura: ");
        io::stdout().flush().expect("Falha ao flushar stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número.");
            }
        }
    }
}
