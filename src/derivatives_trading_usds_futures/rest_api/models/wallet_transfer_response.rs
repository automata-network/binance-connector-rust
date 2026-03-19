use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WalletTransferResponse {
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl WalletTransferResponse {
    #[must_use]
    pub fn new() -> WalletTransferResponse {
        WalletTransferResponse {
            tran_id: None,
            status: None,
        }
    }
}
