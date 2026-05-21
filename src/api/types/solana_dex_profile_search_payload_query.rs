pub use crate::prelude::*;

/// Full-text query used to match profiles by a text value against one or more fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexProfileSearchPayloadQuery {
    /// Text value to match against the selected fields.
    #[serde(default)]
    pub text: String,
    /// Fields that the text value should be matched against.
    #[serde(default)]
    pub fields: Vec<String>,
    /// When true, matches values that are approximately similar to `text`, allowing for minor typos and variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,
    /// When true, matches values that start with `text`, useful for type-ahead lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
}

impl SolanaDexProfileSearchPayloadQuery {
    pub fn builder() -> SolanaDexProfileSearchPayloadQueryBuilder {
        <SolanaDexProfileSearchPayloadQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexProfileSearchPayloadQueryBuilder {
    text: Option<String>,
    fields: Option<Vec<String>>,
    fuzzy: Option<bool>,
    autocomplete: Option<bool>,
}

impl SolanaDexProfileSearchPayloadQueryBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<String>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn fuzzy(mut self, value: bool) -> Self {
        self.fuzzy = Some(value);
        self
    }

    pub fn autocomplete(mut self, value: bool) -> Self {
        self.autocomplete = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexProfileSearchPayloadQuery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SolanaDexProfileSearchPayloadQueryBuilder::text)
    /// - [`fields`](SolanaDexProfileSearchPayloadQueryBuilder::fields)
    pub fn build(self) -> Result<SolanaDexProfileSearchPayloadQuery, BuildError> {
        Ok(SolanaDexProfileSearchPayloadQuery {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            fuzzy: self.fuzzy,
            autocomplete: self.autocomplete,
        })
    }
}
