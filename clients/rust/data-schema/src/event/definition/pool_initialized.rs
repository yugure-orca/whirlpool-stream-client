use super::TokenProgram;
use crate::serde::{big_decimal_as_string, u128_as_string};
use crate::types::{DecimalPrice, PubkeyString, TokenDecimals};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolInitializedEventOrigin,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,
    #[serde(rename = "sp", with = "u128_as_string")]
    pub sqrt_price: u128,
    #[serde(rename = "dp", with = "big_decimal_as_string")]
    pub decimal_price: DecimalPrice,

    #[serde(rename = "c")]
    pub config: PubkeyString,
    #[serde(rename = "tma")]
    pub token_mint_a: PubkeyString,
    #[serde(rename = "tmb")]
    pub token_mint_b: PubkeyString,
    #[serde(rename = "f")]
    pub funder: PubkeyString,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "ft")]
    pub fee_tier: PubkeyString,

    #[serde(rename = "tpa")]
    pub token_program_a: TokenProgram,
    #[serde(rename = "tpb")]
    pub token_program_b: TokenProgram,

    // decimals
    #[serde(rename = "tda")]
    pub token_decimals_a: TokenDecimals,
    #[serde(rename = "tdb")]
    pub token_decimals_b: TokenDecimals,

    // pool state
    #[serde(rename = "cti")]
    pub current_tick_index: i32,
    #[serde(rename = "fr")]
    pub fee_rate: u16,
    #[serde(rename = "pfr")]
    pub protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolInitializedEventOrigin {
    #[serde(rename = "ip")]
    InitializePool,
    #[serde(rename = "ipv2")]
    InitializePoolV2,
}
