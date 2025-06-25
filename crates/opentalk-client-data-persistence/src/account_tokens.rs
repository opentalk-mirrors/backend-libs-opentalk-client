// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The token data required to authorize client requests to the OpenTalk API.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountTokens {
    /// Expiry timestamp of access token
    pub access_token_expiry: DateTime<Utc>,

    /// Access token field
    pub access_token: String,

    /// Refresh token field
    pub refresh_token: String,
}
