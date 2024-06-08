use anyhow::Result;
use relayer_smtp::{config::RelayerSMTPConfig, run};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    run(RelayerSMTPConfig::new()).await
}
