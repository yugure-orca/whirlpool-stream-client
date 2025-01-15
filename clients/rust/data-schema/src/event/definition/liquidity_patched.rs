use crate::serde::u128_as_string;
use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiquidityPatchedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: LiquidityPatchedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ld", with = "u128_as_string")]
    pub liquidity_delta: u128,

    #[serde(rename = "owl", with = "u128_as_string")]
    pub old_whirlpool_liquidity: u128,
    #[serde(rename = "nwl", with = "u128_as_string")]
    pub new_whirlpool_liquidity: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LiquidityPatchedEventOrigin {
    #[serde(rename = "ail")]
    AdminIncreaseLiquidity,
}
