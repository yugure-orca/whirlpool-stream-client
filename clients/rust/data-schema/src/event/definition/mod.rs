mod config_extension_initialized;
mod config_extension_updated;
mod config_initialized;
mod config_updated;
mod fee_tier_initialized;
mod fee_tier_updated;
mod liquidity_deposited;
mod liquidity_patched;
mod liquidity_withdrawn;
mod pool_fee_rate_updated;
mod pool_initialized;
mod pool_protocol_fee_rate_updated;
mod position_bundle_deleted;
mod position_bundle_initialized;
mod position_closed;
mod position_fees_harvested;
mod position_harvest_updated;
mod position_locked;
mod position_opened;
mod position_reward_harvested;
mod protocol_fees_collected;
mod reward_authority_updated;
mod reward_emissions_updated;
mod reward_initialized;
mod tick_array_initialized;
mod token_badge_deleted;
mod token_badge_initialized;
mod traded;

pub use config_extension_initialized::*;
pub use config_extension_updated::*;
pub use config_initialized::*;
pub use config_updated::*;
pub use fee_tier_initialized::*;
pub use fee_tier_updated::*;
pub use liquidity_deposited::*;
pub use liquidity_patched::*;
pub use liquidity_withdrawn::*;
pub use pool_fee_rate_updated::*;
pub use pool_initialized::*;
pub use pool_protocol_fee_rate_updated::*;
pub use position_bundle_deleted::*;
pub use position_bundle_initialized::*;
pub use position_closed::*;
pub use position_fees_harvested::*;
pub use position_harvest_updated::*;
pub use position_locked::*;
pub use position_opened::*;
pub use position_reward_harvested::*;
pub use protocol_fees_collected::*;
pub use reward_authority_updated::*;
pub use reward_emissions_updated::*;
pub use reward_initialized::*;
pub use tick_array_initialized::*;
pub use token_badge_deleted::*;
pub use token_badge_initialized::*;
pub use traded::*;

mod program_deployed;
pub use program_deployed::*;

use crate::serde::{option_u64_as_string, u64_as_string};
use serde_derive::{Deserialize, Serialize};

use crate::types::{PubkeyString, TokenDecimals};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TransferInfo {
    #[serde(rename = "m")]
    pub mint: PubkeyString,

    #[serde(rename = "a", with = "u64_as_string")]
    pub amount: u64,

    #[serde(rename = "d")]
    pub decimals: TokenDecimals,

    #[serde(
        rename = "tfb",
        skip_serializing_if = "Option::is_none",
        default = "Option::default"
    )]
    pub transfer_fee_bps: Option<u16>,
    #[serde(
        rename = "tfm",
        skip_serializing_if = "Option::is_none",
        default = "Option::default",
        with = "option_u64_as_string"
    )]
    pub transfer_fee_max: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TokenProgram {
    #[serde(rename = "t")]
    Token,
    #[serde(rename = "t2")]
    Token2022,
}
