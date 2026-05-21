pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexTokenProfileSearchPayloadQueryField {
    /// Token profile fields that the text value should be matched against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum>>,
    /// Text value to match against the selected fields.
    #[serde(default)]
    pub text: String,
    /// When true, matches values that are approximately similar to `text`, allowing for minor typos and variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,
    /// When true, matches values that start with `text`, useful for type-ahead lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
}

impl SolanaDexTokenProfileSearchPayloadQueryField {
    pub fn builder() -> SolanaDexTokenProfileSearchPayloadQueryFieldBuilder {
        <SolanaDexTokenProfileSearchPayloadQueryFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfileSearchPayloadQueryFieldBuilder {
    fields: Option<Vec<SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum>>,
    text: Option<String>,
    fuzzy: Option<bool>,
    autocomplete: Option<bool>,
}

impl SolanaDexTokenProfileSearchPayloadQueryFieldBuilder {
    pub fn fields(mut self, value: Vec<SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
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

    /// Consumes the builder and constructs a [`SolanaDexTokenProfileSearchPayloadQueryField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SolanaDexTokenProfileSearchPayloadQueryFieldBuilder::text)
    pub fn build(self) -> Result<SolanaDexTokenProfileSearchPayloadQueryField, BuildError> {
        Ok(SolanaDexTokenProfileSearchPayloadQueryField {
            fields: self.fields,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            fuzzy: self.fuzzy,
            autocomplete: self.autocomplete,
        })
    }
}
