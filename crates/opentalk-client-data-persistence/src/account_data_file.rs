// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use serde::{Deserialize, Serialize};

use crate::AccountTokens;

/// A wrapper around the [`AccountData`] for defining the data storage file format.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct AccountDataFile {
    /// The account token data.
    pub opentalk_account_tokens: AccountTokens,
}
