// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{AccountTokens, DataError, OpenTalkInstanceAccountId};

/// The [DataManager] defines the interface for storing and loading data for the OpenTalk client locally
pub trait DataManager: std::fmt::Debug + Sync {
    /// Load instaceData
    fn load_instance(&self, id: &OpenTalkInstanceAccountId) -> Result<AccountTokens, DataError>;

    /// Store instaceData
    fn store_instance(
        &self,
        id: &OpenTalkInstanceAccountId,
        opentalk_account_tokens: AccountTokens,
    ) -> Result<(), DataError>;
}
