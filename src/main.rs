use tracing::{info, span, warn, Level};
use tracing_subscriber;

mod server;
pub mod crud;
pub mod utils;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    server::listen()?;
    Ok(())
}
