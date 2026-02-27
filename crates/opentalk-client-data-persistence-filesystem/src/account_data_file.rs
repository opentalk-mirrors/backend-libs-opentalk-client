// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_client_data_persistence::AccountTokens;
use serde::{Deserialize, Serialize};

/// A wrapper around the [`AccountData`] for defining the data storage file format.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct AccountDataFile {
    /// The account token data.
    pub opentalk_account_tokens: AccountTokens,
}
