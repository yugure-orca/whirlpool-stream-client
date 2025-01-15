use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ofa")]
    pub old_fee_authority: PubkeyString,
    #[serde(rename = "nfa")]
    pub new_fee_authority: PubkeyString,

    #[serde(rename = "ocpfa")]
    pub old_collect_protocol_fees_authority: PubkeyString,
    #[serde(rename = "ncpfa")]
    pub new_collect_protocol_fees_authority: PubkeyString,

    #[serde(rename = "oresa")]
    pub old_reward_emissions_super_authority: PubkeyString,
    #[serde(rename = "nresa")]
    pub new_reward_emissions_super_authority: PubkeyString,

    #[serde(rename = "odpfr")]
    pub old_default_protocol_fee_rate: u16,
    #[serde(rename = "ndpfr")]
    pub new_default_protocol_fee_rate: u16,
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigUpdatedEventOrigin {
    #[serde(rename = "sfa")]
    SetFeeAuthority,
    #[serde(rename = "scpfa")]
    SetCollectProtocolFeesAuthority,
    #[serde(rename = "sresa")]
    SetRewardEmissionsSuperAuthority,
    #[serde(rename = "sdpfr")]
    SetDefaultProtocolFeeRate,
}
