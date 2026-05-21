pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadRequestErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BadRequestErrorBody {
    pub fn builder() -> BadRequestErrorBodyBuilder {
        <BadRequestErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyBuilder {
    error: Option<String>,
}

impl BadRequestErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BadRequestErrorBody`].
    pub fn build(self) -> Result<BadRequestErrorBody, BuildError> {
        Ok(BadRequestErrorBody { error: self.error })
    }
}
