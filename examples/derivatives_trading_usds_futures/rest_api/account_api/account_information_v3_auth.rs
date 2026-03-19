use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::AccountInformationV3Params,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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

    // Setup the API parameters
    let params = AccountInformationV3Params::default();

    // Make the API call
    let response = rest_client
        .account_information_v3(params)
        .await
        .context("account_information_v3 request failed")?;

    info!(?response.rate_limits, "account_information_v3 rate limits");
    let data = response.data().await?;
    info!(?data, "account_information_v3 data");

    Ok(())
}
