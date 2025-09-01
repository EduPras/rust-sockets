use std::io::{self, Write};
use std::net::TcpStream;
use tracing::info;

pub fn start() -> io::Result<()> {
    let payload = build_payload();

    let mut stream = TcpStream::connect("127.0.0.1:60000")?;

    info!("Client: Message sent: {}", payload);

    println!("Client: Message sent. Waiting for a response...");

    Ok(())
}

fn build_payload() -> String {
    let operation = read_operation();
    let product_name = read_product_name();
    let calories = read_calories();
    let carbo = read_carbo();
    let protein = read_protein();
    let fats = read_fats();

    format!(
        "^{}|{}|{}|{}|{}|{}$",
        operation, product_name, calories, carbo, protein, fats
    )
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
