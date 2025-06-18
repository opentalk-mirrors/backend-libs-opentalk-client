// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{opentalk_account_id::OpenTalkAccountId, opentalk_instance_id::OpenTalkInstanceId};

/// Config to store needable state of the opentalk cli
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Config {
    /// Default OpenTalk instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_instance: Option<OpenTalkInstanceId>,

    /// OpenTalk instances
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub instances: BTreeMap<OpenTalkInstanceId, OpenTalkInstance>,
}

/// OpenTalk instance
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkInstance {
    pub default_account: OpenTalkAccountId,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub accounts: BTreeMap<OpenTalkAccountId, OpenTalkAccount>,
}

/// OpenTalkAccount
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkAccount {
    /// COIDC client id
    pub oidc_client_id: String,
}
