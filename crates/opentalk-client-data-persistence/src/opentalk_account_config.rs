// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use serde::{Deserialize, Serialize};

/// Configuration of an OpenTalk account.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpenTalkAccountConfig {
    /// OIDC client id
    pub oidc_client_id: String,
}
