pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SolanaDexProfileSearchPayloadDynamicLabels(
    pub HashMap<String, SolanaDexProfileSearchPayloadFilter>,
);
