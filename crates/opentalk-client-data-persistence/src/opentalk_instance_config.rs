// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{OpenTalkAccountConfig, OpenTalkAccountId};

/// Configuration of an OpenTalk instance.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkInstanceConfig {
    /// The id of the default account to use when using the OpenTalk instance.
    pub default_account: OpenTalkAccountId,

    /// The accounts that are configured for an OpenTalk instance.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub accounts: BTreeMap<OpenTalkAccountId, OpenTalkAccountConfig>,
}
