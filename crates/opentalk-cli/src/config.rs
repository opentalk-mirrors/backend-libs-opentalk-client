use serde::{Deserialize, Serialize};
use url::Url;

/// Config to store needable state of the opentalk cli
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Config {
    /// Default OpenTalk instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_instance_url: Option<Url>,

    /// OpenTalk instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<OpenTalkInstance>,
}

/// OpenTalk instance
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkInstance {
    pub url: Url,
    pub default_account_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<OpenTalkAccount>,
}

/// OpenTalkAccount
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkAccount {
    /// COIDC client id
    pub oidc_client_id: String,

    /// Account name
    pub name: String,
}
