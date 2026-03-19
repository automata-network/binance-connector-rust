use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot::{
    SpotRestApi,
    rest_api::{NewOrderParams, NewOrderSideEnum, NewOrderTimeInForceEnum, NewOrderTypeEnum},
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

    let rest_client = SpotRestApi::production(rest_conf);

    let params = NewOrderParams::builder(
        "ETHUSDT".to_string(),
        NewOrderSideEnum::Buy,
        NewOrderTypeEnum::Limit,
    )
    .quantity(rust_decimal::Decimal::new(5, 3)) // 0.005
    .price(rust_decimal::Decimal::new(1000, 0)) // 1000
    .time_in_force(NewOrderTimeInForceEnum::Gtc)
    .new_client_order_id("abcde12345".to_string())
    .build()?;

    let response = rest_client
        .new_order(params)
        .await
        .context("new_order request failed")?;

    info!(?response.rate_limits, "new_order rate limits");
    let data = response.data().await?;
    info!(?data, "new_order data");

    Ok(())
}
