
use crate::utils::{Item, Column};
use crate::utils::ascii2u8;
use tracing::{debug, error, info, instrument};
/// Create a new item from the provided data
/// 
/// # Arguments
/// * `data` - `Vec<u8>`:
///     The raw bytes representing the item to create
///     * `number of columns`: 1..2 byte (u8)
///     (for each column / first column example)
///         * `column number`: 2..3 bytes (u8)
///         * `column type`: 3..4 bytes (u8)
///         * `column data_length`: 5..6 bytes (u8 -> string max 256 chars)
///         * `column data`: 6..data_length bytes (raw bytes)

#[instrument(skip(data), fields(data_str = %String::from_utf8_lossy(&data)))]
pub fn create_item(data: Vec<u8>) {
    let mut item = Item {
        columns: Vec::new(),
    };
    let n_cols: u8 = ascii2u8(data[1]);
    info!(n_cols);

    let mut cursor: usize = 2;

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