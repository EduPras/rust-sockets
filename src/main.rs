use tracing_subscriber;

pub mod crud;
mod server;
pub mod utils;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    server::listen()?;
    Ok(())
}
