pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<SubscriptionMethodEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<SubscriptionTopicEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<SubscribePayloadParams>,
}

impl SubscribePayload {
    pub fn builder() -> SubscribePayloadBuilder {
        <SubscribePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribePayloadBuilder {
    method: Option<SubscriptionMethodEnum>,
    topic: Option<SubscriptionTopicEnum>,
    params: Option<SubscribePayloadParams>,
}

impl SubscribePayloadBuilder {
    pub fn method(mut self, value: SubscriptionMethodEnum) -> Self {
        self.method = Some(value);
        self
    }

    pub fn topic(mut self, value: SubscriptionTopicEnum) -> Self {
        self.topic = Some(value);
        self
    }

    pub fn params(mut self, value: SubscribePayloadParams) -> Self {
        self.params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribePayload`].
    pub fn build(self) -> Result<SubscribePayload, BuildError> {
        Ok(SubscribePayload {
            method: self.method,
            topic: self.topic,
            params: self.params,
        })
    }
}
