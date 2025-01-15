use super::TransferInfo;
use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionRewardHarvestedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionRewardHarvestedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    // transfer info
    #[serde(rename = "tr")]
    pub transfer_reward: TransferInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionRewardHarvestedEventOrigin {
    #[serde(rename = "cr")]
    CollectReward,
    #[serde(rename = "crv2")]
    CollectRewardV2,
}
