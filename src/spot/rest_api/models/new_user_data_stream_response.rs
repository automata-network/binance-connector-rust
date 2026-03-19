use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewUserDataStreamResponse {
    #[serde(rename = "listenKey", skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<String>,
}
