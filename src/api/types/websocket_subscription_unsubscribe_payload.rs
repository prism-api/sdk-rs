pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnsubscribePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<SubscriptionMethodEnum>,
    /// The topics to unsubscribe from. Leave empty to unsubscribe from all topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<SubscriptionTopicEnum>>,
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
    topics: Option<Vec<SubscriptionTopicEnum>>,
}

impl UnsubscribePayloadBuilder {
    pub fn method(mut self, value: SubscriptionMethodEnum) -> Self {
        self.method = Some(value);
        self
    }

    pub fn topics(mut self, value: Vec<SubscriptionTopicEnum>) -> Self {
        self.topics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribePayload`].
    pub fn build(self) -> Result<UnsubscribePayload, BuildError> {
        Ok(UnsubscribePayload {
            method: self.method,
            topics: self.topics,
        })
    }
}
