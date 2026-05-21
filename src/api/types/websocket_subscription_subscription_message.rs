pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<SubscriptionTopicEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SubscriptionMessageData>,
}

impl SubscriptionMessage {
    pub fn builder() -> SubscriptionMessageBuilder {
        <SubscriptionMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionMessageBuilder {
    topic: Option<SubscriptionTopicEnum>,
    data: Option<SubscriptionMessageData>,
}

impl SubscriptionMessageBuilder {
    pub fn topic(mut self, value: SubscriptionTopicEnum) -> Self {
        self.topic = Some(value);
        self
    }

    pub fn data(mut self, value: SubscriptionMessageData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionMessage`].
    pub fn build(self) -> Result<SubscriptionMessage, BuildError> {
        Ok(SubscriptionMessage {
            topic: self.topic,
            data: self.data,
        })
    }
}
