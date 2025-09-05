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

impl OpenTalkInstanceConfig {
    /// Get the default instance if available.
    pub fn get_default_account(&self) -> Option<(OpenTalkAccountId, OpenTalkAccountConfig)> {
        self.get_account(&self.default_account)
            .map(|account| (self.default_account.clone(), account))
    }

    /// Get an account by its [OpenTalkAccountId].
    pub fn get_account(&self, account_id: &OpenTalkAccountId) -> Option<OpenTalkAccountConfig> {
        self.accounts.get(account_id).cloned()
    }

    /// Get an account config by an optional id, returning the default account if the id is [None].
    pub fn get_account_with_fallback_to_default(
        &self,
        account_id: Option<&OpenTalkAccountId>,
    ) -> Option<(OpenTalkAccountId, OpenTalkAccountConfig)> {
        if let Some(account_id) = account_id {
            let account = self.get_account(account_id)?;
            Some((account_id.clone(), account))
        } else {
            self.get_default_account()
        }
    }

    /// Remove an account from the configuration.
    pub fn remove_account(&mut self, account_id: Option<&OpenTalkAccountId>) {
        let account_id = account_id.unwrap_or(&self.default_account);

        let _ = self.accounts.remove(account_id);
    }
}
