use std::io::Read;
// use std::io::Write;
use std::net::{TcpListener, TcpStream};


// CRUD logic
// Create
fn create_item(data: Vec<u8>) {
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
    println!("===> CREATE");
}

// Read
fn read_item(data: Vec<u8>) {
    /*
    BYTES 
    id: 0..32 bytes (MD5)
     */
    println!("===> DELETE");
}

// Update
fn update_item(data: Vec<u8>) {
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
fn delete_item(data: Vec<u8>) {
    /*
    BYTES 
    id: 0..32 bytes (MD5)
     */
    println!("===> DELETE");
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let operation  = String::from_utf8_lossy(&buffer[0..1]);
    let data = buffer[0..bytes_read].to_vec();
    println!("===> Operation called: {} with {} bytes", operation, bytes_read);
    match operation.as_ref() {
        "C" => create_item(data),
        "R" => read_item(data),
        "U" => update_item(data),
        "D" => delete_item(data),
        _ => println!("===> Unknown operation")
    };
    // println!("===> Received request:\n{}", String::from_utf8_lossy(&buffer[..bytes_read]));
}

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:60000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("\n===> New connection: {}", stream.as_ref().unwrap().peer_addr().unwrap());
        handle_client(stream?);
    }
    Ok(())
}