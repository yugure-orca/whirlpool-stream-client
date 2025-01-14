use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionHarvestUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionHarvestUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionHarvestUpdatedEventOrigin {
    #[serde(rename = "ufar")]
    UpdateFeesAndRewards,
}
