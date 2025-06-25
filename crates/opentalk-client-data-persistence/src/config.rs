// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{opentalk_instance_config::OpenTalkInstanceConfig, OpenTalkInstanceId};

/// Config to store needable state of the opentalk client
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Config {
    /// Default OpenTalk instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_instance: Option<OpenTalkInstanceId>,

    /// OpenTalk instances
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub instances: BTreeMap<OpenTalkInstanceId, OpenTalkInstanceConfig>,
}
