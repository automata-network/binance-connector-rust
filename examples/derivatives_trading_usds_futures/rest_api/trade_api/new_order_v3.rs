use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi,
    rest_api::{NewOrderParams, NewOrderSideEnum, NewOrderTimeInForceEnum},
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

    // Setup the API parameters
    let mut params = NewOrderParams::builder(
        "ETHUSDT".to_string(),
        NewOrderSideEnum::Buy,
        "LIMIT".to_string(),
    );
    params = params.quantity(rust_decimal::Decimal::new(1, 2)); // 0.01
    params = params.price(rust_decimal::Decimal::new(2_000, 0));
    params = params.time_in_force(NewOrderTimeInForceEnum::Gtc);
    let params = params.build()?;

    // Make the API call
    let response = rest_client
        .new_order(params)
        .await
        .context("new_order request failed")?;

    info!(?response.rate_limits, "new_order rate limits");
    let data = response.data().await?;
    info!(?data, "new_order data");

    Ok(())
}
