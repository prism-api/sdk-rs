pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InternalServerErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl InternalServerErrorBody {
    pub fn builder() -> InternalServerErrorBodyBuilder {
        <InternalServerErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalServerErrorBodyBuilder {
    error: Option<String>,
}

impl InternalServerErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InternalServerErrorBody`].
    pub fn build(self) -> Result<InternalServerErrorBody, BuildError> {
        Ok(InternalServerErrorBody { error: self.error })
    }
}
