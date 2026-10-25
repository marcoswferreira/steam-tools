use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Steam Tools v0.1.0...");
    info!("Steam Tools CLI & Bridge ready.");
    Ok(())
}
