pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfileSearchPayloadQuery {
    /// Wallet profile fields that the text value should be matched against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<SolanaDexWalletProfileSearchPayloadQueryTargetsEnum>>,
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

impl SolanaDexWalletProfileSearchPayloadQuery {
    pub fn builder() -> SolanaDexWalletProfileSearchPayloadQueryBuilder {
        <SolanaDexWalletProfileSearchPayloadQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileSearchPayloadQueryBuilder {
    fields: Option<Vec<SolanaDexWalletProfileSearchPayloadQueryTargetsEnum>>,
    text: Option<String>,
    fuzzy: Option<bool>,
    autocomplete: Option<bool>,
}

impl SolanaDexWalletProfileSearchPayloadQueryBuilder {
    pub fn fields(
        mut self,
        value: Vec<SolanaDexWalletProfileSearchPayloadQueryTargetsEnum>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileSearchPayloadQuery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SolanaDexWalletProfileSearchPayloadQueryBuilder::text)
    pub fn build(self) -> Result<SolanaDexWalletProfileSearchPayloadQuery, BuildError> {
        Ok(SolanaDexWalletProfileSearchPayloadQuery {
            fields: self.fields,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            fuzzy: self.fuzzy,
            autocomplete: self.autocomplete,
        })
    }
}
