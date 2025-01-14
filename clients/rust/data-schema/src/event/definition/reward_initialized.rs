use super::TokenProgram;
use crate::types::{PubkeyString, TokenDecimals};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RewardInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: RewardInitializedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    #[serde(rename = "rm")]
    pub reward_mint: PubkeyString,

    #[serde(rename = "rtp")]
    pub reward_token_program: TokenProgram,

    // decimals
    #[serde(rename = "rd")]
    pub reward_decimal: TokenDecimals,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum RewardInitializedEventOrigin {
    #[serde(rename = "ir")]
    InitializeReward,
    #[serde(rename = "irv2")]
    InitializeRewardV2,
}
