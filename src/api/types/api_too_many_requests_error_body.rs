pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TooManyRequestsErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl TooManyRequestsErrorBody {
    pub fn builder() -> TooManyRequestsErrorBodyBuilder {
        <TooManyRequestsErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TooManyRequestsErrorBodyBuilder {
    error: Option<String>,
}

impl TooManyRequestsErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TooManyRequestsErrorBody`].
    pub fn build(self) -> Result<TooManyRequestsErrorBody, BuildError> {
        Ok(TooManyRequestsErrorBody {
            error: self.error,
        })
    }
}
