use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi,
    rest_api::{
        place_multiple_orders_batch_orders_parameter_inner::{SideEnum, TimeInForceEnum},
        PlaceMultipleOrdersBatchOrdersParameterInner, PlaceMultipleOrdersParams,
    },
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

    let order = PlaceMultipleOrdersBatchOrdersParameterInner {
        symbol: Some("ETHUSDT".to_string()),
        side: Some(SideEnum::Buy),
        r#type: Some("LIMIT".to_string()),
        quantity: Some("0.005".to_string()),
        price: Some("1000".to_string()),
        time_in_force: Some(TimeInForceEnum::Gtc),
        new_client_order_id: Some("abcde12345".to_string()),
        ..Default::default()
    };

    let params = PlaceMultipleOrdersParams::builder(vec![order]).build()?;

    let response = rest_client
        .place_multiple_orders(params)
        .await
        .context("place_multiple_orders request failed")?;

    info!(?response.rate_limits, "place_multiple_orders rate limits");
    let data = response.data().await?;
    info!(?data, "place_multiple_orders data");

    Ok(())
}
