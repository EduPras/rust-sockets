use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};

pub fn start() -> io::Result<()> {

    let payload = build_payload();

    let mut stream = TcpStream::connect("127.0.0.1:60000")?;

    stream.write_all(payload.as_bytes())?;

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

    format!("^{operation}|{product_name}|{calories}|{carbo}|{protein}|{fats}$")
}

fn read_operation() -> char {
    loop {
        print!(
            "Selecione a operação:\
    C - create\
    R - read\
    U - update\
    D - delete"
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha.");

        let trimmed_input = input.trim();
        let char_input = trimmed_input.chars().next().unwrap().to_ascii_uppercase();

        match char_input {
            'C' | 'R' | 'U' | 'D' => return char_input,
            _ => {
                println!("Invalid character. Please try again.");
                continue;
            }
        }
    }
}

fn read_product_name() -> String {
    print!("Informe o nome do produto");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input
}

fn read_calories() -> f32 {
    print!("Informe as calorias");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input.parse().expect("Falha ao processar calorias")
}

fn read_carbo() -> f32 {
    print!("Informe o carbo");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input.parse().expect("Falha ao processar calorias")
}

fn read_protein() -> f32 {
    print!("Informe a proteina");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input.parse().expect("Falha ao processar proteinas")
}

fn read_fats() -> f32 {
    print!("Informe a gordura");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha.");

    input.parse().expect("Falha ao processar gordura")
}
