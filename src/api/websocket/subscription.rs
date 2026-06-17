use crate::{ApiError, WebSocketClient, WebSocketMessage, WebSocketOptions, QueryBuilder};
use tokio::sync::{mpsc};
use crate::prelude::{*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscriptionConnectOptions {
    /// Your Prism API key. You can get one for free in the [Prism Dashboard](https://dashboard.prismapi.io).
    #[serde(rename = "x-api-key")]
    pub x_api_key: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SubscriptionServerMessage {
    SubscriptionMessage(SubscriptionMessage),
    /// Unknown or new server message type not yet supported by this SDK version.
    Unknown(serde_json::Value),
}

impl<'de> Deserialize<'de> for SubscriptionServerMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let original_keys: std::collections::BTreeSet<String> = value
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();

        if original_keys.is_empty() {
            return Ok(Self::Unknown(value));
        }

        let mut best_variant: Option<Self> = None;
        let mut best_score: usize = 0;

        if let Ok(v) = serde_json::from_value::<SubscriptionMessage>(value.clone()) {
            if let Ok(re) = serde_json::to_value(&v) {
                let score = re.as_object()
                    .map(|o| o.keys().filter(|k| original_keys.contains(k.as_str())).count())
                    .unwrap_or(0);
                if score > best_score {
                    best_score = score;
                    best_variant = Some(Self::SubscriptionMessage(v));
                }
            }
        }

        let _ = best_score;
        Ok(best_variant.unwrap_or(Self::Unknown(value)))
    }
}
pub struct SubscriptionClient {
    ws: WebSocketClient,
    incoming_rx: mpsc::UnboundedReceiver<Result<WebSocketMessage, ApiError>>,
}
impl SubscriptionClient {
    pub async fn connect(url: &str, authorization: Option<&str>, options: &SubscriptionConnectOptions) -> Result<Self, ApiError> {
        let full_url = format!("{}/v1/ws", url);
        let mut ws_options = WebSocketOptions::default();
        if let Some(auth) = authorization {
            ws_options.headers.insert("Authorization".to_string(), auth.to_string());
        }
        ws_options.query_params = QueryBuilder::new()
            .string("x-api-key", options.x_api_key.clone())
            .build()
            .unwrap_or_default();
        let (ws, incoming_rx) = WebSocketClient::connect(&full_url, ws_options).await?;
        Ok(Self { ws, incoming_rx })
    }

    pub async fn send_subscribe_payload(&self, message: &SubscribePayload) -> Result<(), ApiError> {
        self.ws.send_json(message).await
    }

    pub async fn send_unsubscribe_payload(&self, message: &UnsubscribePayload) -> Result<(), ApiError> {
        self.ws.send_json(message).await
    }

    pub async fn recv(&mut self) -> Option<Result<SubscriptionServerMessage, ApiError>> {
        match self.incoming_rx.recv().await {
            Some(Ok(WebSocketMessage::Text(raw))) => {
                Some(serde_json::from_str::<SubscriptionServerMessage>(&raw).map_err(ApiError::Serialization))
            }
            Some(Ok(WebSocketMessage::Binary(data))) => {
                Some(Err(ApiError::WebSocketError(
                    format!("Received unexpected binary frame ({} bytes) on a JSON-only channel", data.len())
                )))
            }
            Some(Ok(WebSocketMessage::Close(_))) => None,
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    pub async fn close(&self) -> Result<(), ApiError> {
        self.ws.close().await
    }
}
/// Connector for the Subscription WebSocket channel.
/// Provides access to the WebSocket channel through the root client.
pub struct SubscriptionConnector {
    base_url: String,
    auth_header: Option<String>,
}

impl SubscriptionConnector {
    pub fn new(base_url: String, auth_header: Option<String>) -> Self {
        Self { base_url, auth_header }
    }

    pub async fn connect(&self, options: &SubscriptionConnectOptions) -> Result<SubscriptionClient, ApiError> {
        SubscriptionClient::connect(&self.base_url, self.auth_header.as_deref(), options).await
    }
}
