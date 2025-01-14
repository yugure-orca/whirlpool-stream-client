use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigExtensionUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigExtensionUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyString,

    #[serde(rename = "ocea")]
    pub old_config_extension_authority: PubkeyString,
    #[serde(rename = "ncea")]
    pub new_config_extension_authority: PubkeyString,

    #[serde(rename = "otba")]
    pub old_token_badge_authority: PubkeyString,
    #[serde(rename = "ntba")]
    pub new_token_badge_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigExtensionUpdatedEventOrigin {
    #[serde(rename = "scea")]
    SetConfigExtensionAuthority,
    #[serde(rename = "stba")]
    SetTokenBadgeAuthority,
}
