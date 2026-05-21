pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct SolanaDexTokenProfileLabels(pub Vec<SolanaDexTokenProfileLabelEnum>);