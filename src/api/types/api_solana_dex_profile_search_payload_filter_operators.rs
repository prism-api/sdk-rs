pub use crate::prelude::*;

/// Comparison operators applied to a single field. At least one operator must be provided.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexProfileSearchPayloadFilterOperators {
    /// Equal to.
    #[serde(rename = "$eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<SolanaDexProfileSearchPayloadFilterOperatorsEq>,
    /// Not equal to.
    #[serde(rename = "$ne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne: Option<SolanaDexProfileSearchPayloadFilterOperatorsNe>,
    /// Greater than.
    #[serde(rename = "$gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gt: Option<f64>,
    /// Less than.
    #[serde(rename = "$lt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lt: Option<f64>,
    /// Greater than or equal to.
    #[serde(rename = "$gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gte: Option<f64>,
    /// Less than or equal to.
    #[serde(rename = "$lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lte: Option<f64>,
    /// In the list.
    #[serde(rename = "$in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<SolanaDexProfileSearchPayloadFilterOperatorsInItem>>,
    /// Not in the list.
    #[serde(rename = "$nin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nin: Option<Vec<SolanaDexProfileSearchPayloadFilterOperatorsNinItem>>,
}

impl SolanaDexProfileSearchPayloadFilterOperators {
    pub fn builder() -> SolanaDexProfileSearchPayloadFilterOperatorsBuilder {
        <SolanaDexProfileSearchPayloadFilterOperatorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexProfileSearchPayloadFilterOperatorsBuilder {
    eq: Option<SolanaDexProfileSearchPayloadFilterOperatorsEq>,
    ne: Option<SolanaDexProfileSearchPayloadFilterOperatorsNe>,
    gt: Option<f64>,
    lt: Option<f64>,
    gte: Option<f64>,
    lte: Option<f64>,
    r#in: Option<Vec<SolanaDexProfileSearchPayloadFilterOperatorsInItem>>,
    nin: Option<Vec<SolanaDexProfileSearchPayloadFilterOperatorsNinItem>>,
}

impl SolanaDexProfileSearchPayloadFilterOperatorsBuilder {
    pub fn eq(mut self, value: SolanaDexProfileSearchPayloadFilterOperatorsEq) -> Self {
        self.eq = Some(value);
        self
    }

    pub fn ne(mut self, value: SolanaDexProfileSearchPayloadFilterOperatorsNe) -> Self {
        self.ne = Some(value);
        self
    }

    pub fn gt(mut self, value: f64) -> Self {
        self.gt = Some(value);
        self
    }

    pub fn lt(mut self, value: f64) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn gte(mut self, value: f64) -> Self {
        self.gte = Some(value);
        self
    }

    pub fn lte(mut self, value: f64) -> Self {
        self.lte = Some(value);
        self
    }

    pub fn r#in(mut self, value: Vec<SolanaDexProfileSearchPayloadFilterOperatorsInItem>) -> Self {
        self.r#in = Some(value);
        self
    }

    pub fn nin(mut self, value: Vec<SolanaDexProfileSearchPayloadFilterOperatorsNinItem>) -> Self {
        self.nin = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexProfileSearchPayloadFilterOperators`].
    pub fn build(self) -> Result<SolanaDexProfileSearchPayloadFilterOperators, BuildError> {
        Ok(SolanaDexProfileSearchPayloadFilterOperators {
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            gte: self.gte,
            lte: self.lte,
            r#in: self.r#in,
            nin: self.nin,
        })
    }
}
