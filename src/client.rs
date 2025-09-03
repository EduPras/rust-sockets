use md5::{Digest, Md5};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use tracing::{error, info};
use crate::server_response_handler::handle_server_response;

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

fn build_payload() -> String {
    let operation = read_operation();
    let mut product_name = read_product_name();
    let id = build_id_from(&mut product_name);

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

fn build_id_from(product_name: &mut String) -> String {
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
    io::stdout().flush().expect("Fail to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");

    input
        .trim()
        .replace(|c: char| "^$|".contains(c), "")
        .to_string()
}

fn read_float(prompt: &str) -> f32 {
    loop {
        print!("Informe {}: ", prompt);
        io::stdout().flush().expect("Fail to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail to read line");

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Entrada inválida. Por favor, insira um número.");
            }
        }
    }
}