use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot::{SpotRestApi, rest_api::DepthParams};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let user = env::var("ASTER_USER").context("ASTER_USER must be set")?;
    let signer = env::var("ASTER_SIGNER").context("ASTER_SIGNER must be set")?;
    let signer_private_key =
        env::var("ASTER_SIGNER_PRIVATE_KEY").context("ASTER_SIGNER_PRIVATE_KEY must be set")?;

    let rest_conf = ConfigurationRestApi::builder()
        .user(user)
        .signer(signer)
        .signer_private_key(signer_private_key)
        .build()?;

    let rest_client = SpotRestApi::production(rest_conf);

    let params = DepthParams::builder("BNBUSDT".to_string()).build()?;

    let response = rest_client
        .depth(params)
        .await
        .context("depth request failed")?;

    info!(?response.rate_limits, "depth rate limits");
    let data = response.data().await?;
    info!(?data, "depth data");

    Ok(())
}
