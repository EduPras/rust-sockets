use std::io::Read;
use tracing::{debug, error, info, instrument, span, warn, Level};
// use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn ascii2u8(ascii: u8) -> u8 {
    ascii - b'0'
}

// CRUD logic
#[derive(Debug)]
struct Column {
    col_type: u8, // 0 = integer, 1 = string, 2 = float
    data: Vec<u8>,
}
struct Item {
    id: String, // MD5 hash
    columns: Vec<Column>,
}
/// Create a new item from the provided data
/// 
/// # Arguments
/// * `data` - `Vec<u8>`:
///     The raw bytes representing the item to create
///     * `number of columns`: 33..34 byte (u8)
///     (for each column / first column example)
///         * `column number`: 34..35 bytes (u8)
///         * `column type`: 35..36 bytes (u8)
///         * `column data_length`: 36..37 bytes (u8 -> string max 256 chars)
///         * `column data`: 37..data_length bytes (raw bytes)
///
#[instrument(skip(data), fields(data_str = %String::from_utf8_lossy(&data)))]
fn create_item(data: Vec<u8>) {
    let mut item = Item {
        id: String::from_utf8_lossy(&data[1..33]).into(),
        columns: Vec::new(),
    };
    let n_cols: u8 = ascii2u8(data[33]);
    info!(item.id, n_cols);

    let mut cursor: usize = 34;

    for _ in 0..n_cols {
        if cursor + 3 > data.len() {
            error!("Malformed data. Not enough bytes for column metadata.");
            break; 
        }

        // ##### DEBUG #####
        let col_number = ascii2u8(data[cursor]);
        let col_type = ascii2u8(data[(cursor + 1)]);
        let data_length = ascii2u8(data[(cursor + 2)]);
        let raw_data = ascii2u8(data[(cursor + 3)]);
        debug!(col_number, col_type, data_length, raw_data);
        // #################

        let data_start = cursor + 3 ;
        let data_end = data_start + data_length as usize;

        if data_end as usize > data.len() {
            error!("Malformed data. Column data_length exceeds buffer size.");
            break;
        }

        let col = Column {
            col_type,
            data: data[data_start .. data_end].to_vec(),
        };
        info!(col_type = col.col_type, data = ?col.data, "ADDED");
        item.columns.push(col);

        cursor = data_end as usize;
    }

    debug!("CREATE");
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
    info!("Operation called: {} with {} bytes", operation, bytes_read);
    match operation.as_ref() {
        "C" => create_item(data),
        "R" => read_item(data),
        "U" => update_item(data),
        "D" => delete_item(data),
        _ => warn!("===> Unknown operation")
    };
    // println!("===> Received request:\n{}", String::from_utf8_lossy(&buffer[..bytes_read]));
}

#[instrument]
pub fn listen() -> std::io::Result<u8> {
    let listener = TcpListener::bind("127.0.0.1:60000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_client(s),
            Err(e) => {
                error!("Connection failed: {}", e);
                continue;
            }
        }
    }
    Ok(200)
}