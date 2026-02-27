// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{AccountTokens, DataError};

/// The [DataManager] defines the interface for storing and loading data for the OpenTalk client locally
pub trait DataManager: std::fmt::Debug + Sync {
    /// Load the account tokens
    fn load_account_tokens(&self) -> Result<AccountTokens, DataError>;

    /// Store the account tokens
    fn store_account_tokens(&self, opentalk_account_tokens: AccountTokens)
    -> Result<(), DataError>;
}
