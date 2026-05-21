pub use crate::prelude::*;

/// Conditions that a profile must satisfy to be included in the results.
/// Each key is either a field path paired with comparison operators, or a logical operator (`$and`, `$or`) that combines nested filter objects.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexProfileSearchPayloadFilter {
    /// Logical AND. A profile must satisfy every nested filter to match.
    #[serde(rename = "$and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<SolanaDexProfileSearchPayloadFilter>>>,
    /// Logical OR. A profile must satisfy at least one nested filter to match.
    #[serde(rename = "$or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<SolanaDexProfileSearchPayloadFilter>>>,
}

impl SolanaDexProfileSearchPayloadFilter {
    pub fn builder() -> SolanaDexProfileSearchPayloadFilterBuilder {
        <SolanaDexProfileSearchPayloadFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexProfileSearchPayloadFilterBuilder {
    and: Option<Vec<Box<SolanaDexProfileSearchPayloadFilter>>>,
    or: Option<Vec<Box<SolanaDexProfileSearchPayloadFilter>>>,
}

impl SolanaDexProfileSearchPayloadFilterBuilder {
    pub fn and(mut self, value: Vec<Box<SolanaDexProfileSearchPayloadFilter>>) -> Self {
        self.and = Some(value);
        self
    }

    pub fn or(mut self, value: Vec<Box<SolanaDexProfileSearchPayloadFilter>>) -> Self {
        self.or = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexProfileSearchPayloadFilter`].
    pub fn build(self) -> Result<SolanaDexProfileSearchPayloadFilter, BuildError> {
        Ok(SolanaDexProfileSearchPayloadFilter {
            and: self.and,
            or: self.or,
        })
    }
}
