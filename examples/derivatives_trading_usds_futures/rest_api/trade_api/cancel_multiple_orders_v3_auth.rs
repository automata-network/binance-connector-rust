use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi, rest_api::CancelMultipleOrdersParams,
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

    let params = CancelMultipleOrdersParams::builder("ETHUSDT".to_string())
        .orig_client_order_id_list(vec!["abcde12345".to_string()])
        .build()?;

    let response = rest_client
        .cancel_multiple_orders(params)
        .await
        .context("cancel_multiple_orders request failed")?;

    info!(?response.rate_limits, "cancel_multiple_orders rate limits");
    let data = response.data().await?;
    info!(?data, "cancel_multiple_orders data");

    Ok(())
}
