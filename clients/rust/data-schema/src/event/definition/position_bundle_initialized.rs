use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionBundleInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionBundleInitializedEventOrigin,

    #[serde(rename = "pb")]
    pub position_bundle: PubkeyString,

    #[serde(rename = "pbm")]
    pub position_bundle_mint: PubkeyString,

    #[serde(rename = "pbo")]
    pub position_bundle_owner: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionBundleInitializedEventOrigin {
    #[serde(rename = "ipb")]
    InitializePositionBundle,
    #[serde(rename = "ipbwm")]
    InitializePositionBundleWithMetadata,
}
