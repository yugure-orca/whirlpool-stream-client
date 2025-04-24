use crate::serde::{u128_as_string, big_decimal_as_string};
use crate::types::{DecimalPrice, PubkeyString};
use serde_derive::{Serialize, Deserialize};
use super::PositionLockType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionLockedTransferredEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionLockedTransferredEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,

    #[serde(rename = "lt")]
    pub lock_type: PositionLockType,
    #[serde(rename = "lc")]
    pub lock_config: PubkeyString,

    #[serde(rename = "lti")]
    pub lower_tick_index: i32,
    #[serde(rename = "uti")]
    pub upper_tick_index: i32,
    #[serde(rename = "ldp", with = "big_decimal_as_string")]
    pub lower_decimal_price: DecimalPrice,
    #[serde(rename = "udp", with = "big_decimal_as_string")]
    pub upper_decimal_price: DecimalPrice,

    #[serde(rename = "ll", with = "u128_as_string")]
    pub locked_liquidity: u128,

    #[serde(rename = "opo")]
    pub old_position_owner: PubkeyString,
    #[serde(rename = "npo")]
    pub new_position_owner: PubkeyString,

    #[serde(rename = "pm")]
    pub position_mint: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionLockedTransferredEventOrigin {
    #[serde(rename = "tlp")]
    TranssferLockedPosition,
}
