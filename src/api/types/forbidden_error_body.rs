pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForbiddenErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ForbiddenErrorBody {
    pub fn builder() -> ForbiddenErrorBodyBuilder {
        <ForbiddenErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForbiddenErrorBodyBuilder {
    error: Option<String>,
}

impl ForbiddenErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForbiddenErrorBody`].
    pub fn build(self) -> Result<ForbiddenErrorBody, BuildError> {
        Ok(ForbiddenErrorBody { error: self.error })
    }
}
