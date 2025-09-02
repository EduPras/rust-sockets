use tracing_subscriber;
use crate::utils::Item;

mod server;
mod utils;

mod client;
mod repository;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    // server::listen();
    client::start()?;
    Ok(())
}
