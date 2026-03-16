use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::NoopParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load V3 credentials from env
    let user = env::var("ASTER_USER").context("ASTER_USER must be set")?;
    let signer = env::var("ASTER_SIGNER").context("ASTER_SIGNER must be set")?;
    let signer_private_key =
        env::var("ASTER_SIGNER_PRIVATE_KEY").context("ASTER_SIGNER_PRIVATE_KEY must be set")?;

    // Build REST config with V3 auth
    let rest_conf = ConfigurationRestApi::builder()
        .user(user)
        .signer(signer)
        .signer_private_key(signer_private_key)
        .build()?;

    // Create the DerivativesTradingUsdsFutures REST API client
    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    // Send a noop with a specific nonce (to cancel a pending order that used this nonce)
    let target_nonce: u64 = 1700000000000000; // example nonce
    let params = NoopParams::new().nonce(target_nonce);

    let response = rest_client
        .noop(params)
        .await
        .context("noop request failed")?;

    info!(?response.rate_limits, "noop rate limits");
    let data = response.data().await?;
    info!(?data, "noop data");

    Ok(())
}
