use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "fa")]
    pub fee_authority: PubkeyString,

    #[serde(rename = "cpfa")]
    pub collect_protocol_fees_authority: PubkeyString,

    #[serde(rename = "resa")]
    pub reward_emissions_super_authority: PubkeyString,

    #[serde(rename = "dpfr")]
    pub default_protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigInitializedEventOrigin {
    #[serde(rename = "ic")]
    InitializeConfig,
}
