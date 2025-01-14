use super::TransferInfo;
use crate::serde::{big_decimal_as_string, u128_as_string};
use crate::types::{DecimalPrice, PubkeyString};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TradedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: TradedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "ta")]
    pub token_authority: PubkeyString,

    #[serde(rename = "tm")]
    pub trade_mode: TradeMode,
    #[serde(rename = "td")]
    pub trade_direction: TradeDirection,

    // transfer info
    #[serde(rename = "ti")]
    pub transfer_in: TransferInfo,
    #[serde(rename = "to")]
    pub transfer_out: TransferInfo,

    // pool state
    #[serde(rename = "osp", with = "u128_as_string")]
    pub old_sqrt_price: u128,
    #[serde(rename = "nsp", with = "u128_as_string")]
    pub new_sqrt_price: u128,
    #[serde(rename = "octi")]
    pub old_current_tick_index: i32,
    #[serde(rename = "ncti")]
    pub new_current_tick_index: i32,
    #[serde(rename = "odp", with = "big_decimal_as_string")]
    pub old_decimal_price: DecimalPrice,
    #[serde(rename = "ndp", with = "big_decimal_as_string")]
    pub new_decimal_price: DecimalPrice,
    #[serde(rename = "fr")]
    pub fee_rate: u16,
    #[serde(rename = "pfr")]
    pub protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TradedEventOrigin {
    #[serde(rename = "s")]
    Swap,
    #[serde(rename = "sv2")]
    SwapV2,
    #[serde(rename = "thso")]
    TwoHopSwapOne,
    #[serde(rename = "thst")]
    TwoHopSwapTwo,
    #[serde(rename = "thsv2o")]
    TwoHopSwapV2One,
    #[serde(rename = "thsv2t")]
    TwoHopSwapV2Two,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TradeMode {
    #[serde(rename = "ei")]
    ExactInput,
    #[serde(rename = "eo")]
    ExactOutput,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TradeDirection {
    #[serde(rename = "ab")]
    AtoB,
    #[serde(rename = "ba")]
    BtoA,
}
