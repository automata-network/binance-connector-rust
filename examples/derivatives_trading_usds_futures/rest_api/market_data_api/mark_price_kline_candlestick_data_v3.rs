use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesRestApi,
    rest_api::{MarkPriceKlineCandlestickDataIntervalEnum, MarkPriceKlineCandlestickDataParams},
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

    let params = MarkPriceKlineCandlestickDataParams::builder(
        "BTCUSDT".to_string(),
        MarkPriceKlineCandlestickDataIntervalEnum::Interval1m,
    )
    .build()?;

    let response = rest_client
        .mark_price_kline_candlestick_data(params)
        .await
        .context("mark_price_kline_candlestick_data request failed")?;

    info!(?response.rate_limits, "mark_price_kline_candlestick_data rate limits");
    let data = response.data().await?;
    info!(?data, "mark_price_kline_candlestick_data data");

    Ok(())
}
