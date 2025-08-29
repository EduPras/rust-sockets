use tracing::{info, span, warn, Level};
use tracing_subscriber;

mod server;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    server::listen()?;
    Ok(())
}
