use super::TransferInfo;
use crate::serde::{big_decimal_as_string, u128_as_string};
use crate::types::{DecimalPrice, PubkeyString};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiquidityDepositedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: LiquidityDepositedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,
    #[serde(rename = "lta")]
    pub lower_tick_array: PubkeyString,
    #[serde(rename = "uta")]
    pub upper_tick_array: PubkeyString,

    #[serde(rename = "ld", with = "u128_as_string")]
    pub liquidity_delta: u128,

    // transfer info
    #[serde(rename = "ta")]
    pub transfer_a: TransferInfo,
    #[serde(rename = "tb")]
    pub transfer_b: TransferInfo,

    // position state
    #[serde(rename = "lti")]
    pub lower_tick_index: i32,
    #[serde(rename = "uti")]
    pub upper_tick_index: i32,
    #[serde(rename = "ldp", with = "big_decimal_as_string")]
    pub lower_decimal_price: DecimalPrice,
    #[serde(rename = "udp", with = "big_decimal_as_string")]
    pub upper_decimal_price: DecimalPrice,
    #[serde(rename = "opl", with = "u128_as_string")]
    pub old_position_liquidity: u128,
    #[serde(rename = "npl", with = "u128_as_string")]
    pub new_position_liquidity: u128,

    // pool state
    #[serde(rename = "owl", with = "u128_as_string")]
    pub old_whirlpool_liquidity: u128,
    #[serde(rename = "nwl", with = "u128_as_string")]
    pub new_whirlpool_liquidity: u128,
    #[serde(rename = "wsp", with = "u128_as_string")]
    pub whirlpool_sqrt_price: u128,
    #[serde(rename = "wcti")]
    pub whirlpool_current_tick_index: i32,
    #[serde(rename = "wdp", with = "big_decimal_as_string")]
    pub whirlpool_decimal_price: DecimalPrice,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LiquidityDepositedEventOrigin {
    #[serde(rename = "il")]
    IncreaseLiquidity,
    #[serde(rename = "ilv2")]
    IncreaseLiquidityV2,
}
