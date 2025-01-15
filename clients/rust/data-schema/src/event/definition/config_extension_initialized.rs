use crate::types::PubkeyString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigExtensionInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigExtensionInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyString,

    #[serde(rename = "cea")]
    pub config_extension_authority: PubkeyString,

    #[serde(rename = "tba")]
    pub token_badge_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigExtensionInitializedEventOrigin {
    #[serde(rename = "ice")]
    InitializeConfigExtension,
}
