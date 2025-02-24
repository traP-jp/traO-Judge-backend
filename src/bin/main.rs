use anyhow::Ok;
use tracing_subscriber::EnvFilter;

use trao_judge_backend as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    lib::run().await?;

    Ok(())
}
