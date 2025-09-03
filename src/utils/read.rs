use std::io;
use std::io::Write;

pub fn read_operation() -> char {
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

pub fn read_product_name() -> String {
    print!("Informe o nome do produto: ");
    io::stdout().flush().expect("Fail to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");

    input
        .trim()
        .replace(|c: char| "^$|".contains(c), "")
        .to_string()
}

pub fn read_float(prompt: &str) -> f32 {
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