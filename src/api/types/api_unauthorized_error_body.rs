pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnauthorizedErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl UnauthorizedErrorBody {
    pub fn builder() -> UnauthorizedErrorBodyBuilder {
        <UnauthorizedErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnauthorizedErrorBodyBuilder {
    error: Option<String>,
}

impl UnauthorizedErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnauthorizedErrorBody`].
    pub fn build(self) -> Result<UnauthorizedErrorBody, BuildError> {
        Ok(UnauthorizedErrorBody {
            error: self.error,
        })
    }
}
