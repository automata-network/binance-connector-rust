#![allow(unused_imports)]
use crate::spot::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaxNotionalFilter {
    #[serde(rename = "filterType", skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "minNotional", skip_serializing_if = "Option::is_none")]
    pub min_notional: Option<String>,
    #[serde(rename = "applyMinToMarket", skip_serializing_if = "Option::is_none")]
    pub apply_min_to_market: Option<bool>,
    #[serde(rename = "maxNotional", skip_serializing_if = "Option::is_none")]
    pub max_notional: Option<String>,
    #[serde(rename = "applyMaxToMarket", skip_serializing_if = "Option::is_none")]
    pub apply_max_to_market: Option<bool>,
    #[serde(rename = "avgPriceMins", skip_serializing_if = "Option::is_none")]
    pub avg_price_mins: Option<i32>,
}

impl MaxNotionalFilter {
    #[must_use]
    pub fn new() -> MaxNotionalFilter {
        MaxNotionalFilter {
            filter_type: None,
            min_notional: None,
            apply_min_to_market: None,
            max_notional: None,
            apply_max_to_market: None,
            avg_price_mins: None,
        }
    }
}
