use std::io::{self, Read, Write};
use std::net::TcpStream;
use tracing::{error, info};

use crate::server_response_handler::handle_server_response;
use crate::payload;

pub fn start() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:60000")?;
    info!("Client: Connected to server.");

    loop {
        let payload = payload::request();

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