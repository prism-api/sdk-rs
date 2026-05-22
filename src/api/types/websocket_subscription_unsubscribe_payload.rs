pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnsubscribePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<SubscriptionMethodEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<SubscriptionTopicEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<UnsubscribePayloadParams>,
}

impl UnsubscribePayload {
    pub fn builder() -> UnsubscribePayloadBuilder {
        <UnsubscribePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribePayloadBuilder {
    method: Option<SubscriptionMethodEnum>,
    topic: Option<SubscriptionTopicEnum>,
    params: Option<UnsubscribePayloadParams>,
}

impl UnsubscribePayloadBuilder {
    pub fn method(mut self, value: SubscriptionMethodEnum) -> Self {
        self.method = Some(value);
        self
    }

    pub fn topic(mut self, value: SubscriptionTopicEnum) -> Self {
        self.topic = Some(value);
        self
    }

    pub fn params(mut self, value: UnsubscribePayloadParams) -> Self {
        self.params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribePayload`].
    pub fn build(self) -> Result<UnsubscribePayload, BuildError> {
        Ok(UnsubscribePayload {
            method: self.method,
            topic: self.topic,
            params: self.params,
        })
    }
}
