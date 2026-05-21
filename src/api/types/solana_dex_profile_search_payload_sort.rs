pub use crate::prelude::*;

/// Rule that determines the order in which matching profiles are returned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SolanaDexProfileSearchPayloadSort {
    /// Field path to sort results by.
    #[serde(default)]
    pub field: String,
    pub direction: SolanaDexProfileSearchPayloadSortDirectionEnum,
}

impl SolanaDexProfileSearchPayloadSort {
    pub fn builder() -> SolanaDexProfileSearchPayloadSortBuilder {
        <SolanaDexProfileSearchPayloadSortBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexProfileSearchPayloadSortBuilder {
    field: Option<String>,
    direction: Option<SolanaDexProfileSearchPayloadSortDirectionEnum>,
}

impl SolanaDexProfileSearchPayloadSortBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn direction(mut self, value: SolanaDexProfileSearchPayloadSortDirectionEnum) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexProfileSearchPayloadSort`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SolanaDexProfileSearchPayloadSortBuilder::field)
    /// - [`direction`](SolanaDexProfileSearchPayloadSortBuilder::direction)
    pub fn build(self) -> Result<SolanaDexProfileSearchPayloadSort, BuildError> {
        Ok(SolanaDexProfileSearchPayloadSort {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
        })
    }
}
