use std::io;
use std::io::Write;
use tracing_subscriber;

mod server;
mod client;
mod repository;
mod server_response_handler;
mod payload;
mod utils;

fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    match select_mode() {
        'S' => server::listen(),
        'C' => client::start()?,
        _ => {}
    }

    Ok(())
}

fn select_mode() -> char {
    loop {
        print!(
            "Selecione o modo:\n\
            C - cliente\n\
            S - server\n> "
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
                'C' | 'S' => return upper_char,
                _ => println!("Caractere inválido. Tente novamente."),
            }
        } else {
            println!("Entrada vazia. Tente novamente.");
        }
    }
}
