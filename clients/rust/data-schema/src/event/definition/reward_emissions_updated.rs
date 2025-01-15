use crate::serde::u128_as_string;
use crate::types::{PubkeyString, TokenDecimals};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RewardEmissionsUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: RewardEmissionsUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    #[serde(rename = "rm")]
    pub reward_mint: PubkeyString,

    #[serde(rename = "rd")]
    pub reward_decimals: TokenDecimals,

    #[serde(rename = "oepsx64", with = "u128_as_string")]
    pub old_emissions_per_second_x64: u128,

    #[serde(rename = "nepsx64", with = "u128_as_string")]
    pub new_emissions_per_second_x64: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum RewardEmissionsUpdatedEventOrigin {
    #[serde(rename = "sre")]
    SetRewardEmissions,
    #[serde(rename = "srev2")]
    SetRewardEmissionsV2,
}
