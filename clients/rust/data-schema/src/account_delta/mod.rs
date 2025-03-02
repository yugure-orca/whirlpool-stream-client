use crate::serde::vec_u8_as_base64_string;
use crate::types::{BlockHeight, BlockTime, Bytes, PubkeyString, Slot};
use serde_derive::{Deserialize, Serialize};

/*

Whirlpool Account Delta JSON Lines Format

To reduce data size, we use short field names.
Each line is a JSON object with the following schema:

{
  slot(s): u64,
  blockHeight(h): u64,
  blockTime(t): i64,
  accountDeltas(a): [
    {
      pubkey(p): String(base58 encoding),
      accountType(t): String,
      delta(d): "I",
      length(l): u16,
      segments(s): [
        { offset(o): u16, data(d): String(base64 encoding) },
        { offset(o): u16, data(d): String(base64 encoding) },
        ...
      ]
    },
    {
      pubkey(p): String(base58 encoding),
      accountType(t): String,
      delta(d): "U",
      segments(s): [
        { offset(o): u16, data(d): String(base64 encoding) },
        { offset(o): u16, data(d): String(base64 encoding) },
        ...
      ]
    },
    {
      pubkey(p): String(base58 encoding),
      accountType(t): String,
      delta(d): "C",
    },
    ...
  ]
}

*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockWhirlpoolAccountDelta {
    #[serde(rename = "s")]
    pub slot: Slot,
    #[serde(rename = "h")]
    pub block_height: BlockHeight,
    #[serde(rename = "t")]
    pub block_time: BlockTime,
    #[serde(rename = "a")]
    pub account_deltas: Vec<WhirlpoolAccountDelta>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WhirlpoolAccountDelta {
    #[serde(rename = "p")]
    pub pubkey: PubkeyString,
    #[serde(rename = "t")]
    pub account_type: WhirlpoolAccountType,
    #[serde(flatten)]
    pub delta: AccountDataDelta,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "d")]
pub enum AccountDataDelta {
    #[serde(rename = "I")]
    Initialized {
        #[serde(rename = "l")]
        length: u16,
        #[serde(rename = "s")]
        segments: Vec<DataDeltaSegment>,
    },
    #[serde(rename = "U")]
    Updated {
        #[serde(rename = "s")]
        segments: Vec<DataDeltaSegment>,
    },
    #[serde(rename = "C")]
    Closed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DataDeltaSegment {
    #[serde(rename = "o")]
    pub offset: u16,
    #[serde(rename = "d", with = "vec_u8_as_base64_string")]
    pub data: Bytes,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WhirlpoolAccountType {
    #[serde(rename = "C")]
    Config,
    #[serde(rename = "CE")]
    ConfigExtension,
    #[serde(rename = "FT")]
    FeeTier,
    #[serde(rename = "W")]
    Whirlpool,
    #[serde(rename = "TA")]
    TickArray,
    #[serde(rename = "P")]
    Position,
    #[serde(rename = "PB")]
    PositionBundle,
    #[serde(rename = "TB")]
    TokenBadge,
    #[serde(rename = "LC")]
    LockConfig,
}
