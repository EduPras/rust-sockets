use tracing_subscriber;

pub mod crud;
mod server;
pub mod utils;

mod client;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    server::listen()?;
    Ok(())
}
