use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::FuturesAccountBalanceV3Params,
};

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

    let rest_client = DerivativesTradingUsdsFuturesRestApi::production(rest_conf);

    let params = FuturesAccountBalanceV3Params::default();

    let response = rest_client
        .futures_account_balance_v3(params)
        .await
        .context("futures_account_balance_v3 request failed")?;

    info!(?response.rate_limits, "futures_account_balance_v3 rate limits");
    let data = response.data().await?;
    info!(?data, "futures_account_balance_v3 data");

    Ok(())
}
