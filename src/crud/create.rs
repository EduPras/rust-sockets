
use crate::utils::{Item};
use crate::utils::ascii2u8;
use tracing::{debug, error, info, instrument};
/// Create a new item from the provided data
/// 
/// # Arguments
/// * `item` - `Item`:
#[instrument]
pub fn create_item(item: Item) -> std::io::Result<(u32)>{
    // Connect to the database and save it
    Ok(200)
}