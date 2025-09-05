// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    OpenTalkAccountConfig, OpenTalkAccountId, OpenTalkInstanceAccountId, OpenTalkInstanceId,
    opentalk_instance_config::OpenTalkInstanceConfig,
};

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

impl Config {
    /// Get the default instance if available.
    pub fn get_default_instance(&self) -> Option<(OpenTalkInstanceId, OpenTalkInstanceConfig)> {
        let instance_id = self.default_instance.clone()?;
        let instance = self.get_instance(&instance_id)?.clone();
        Some((instance_id, instance))
    }

    /// Get an instance by its [OpenTalkInstanceId].
    pub fn get_instance(&self, instance_id: &OpenTalkInstanceId) -> Option<OpenTalkInstanceConfig> {
        self.instances.get(instance_id).cloned()
    }

    /// Get the default account if available.
    ///
    /// This will return the default account of the default instance.
    pub fn get_default_account(
        &self,
    ) -> Option<(OpenTalkInstanceAccountId, OpenTalkAccountConfig)> {
        let (instance_id, instance) = self.get_default_instance()?;

        let (account_id, account) = instance.get_default_account()?;

        Some(((instance_id.clone(), account_id).into(), account))
    }

    /// Get an instance config by an optional id, returning the default config if the id is [None].
    pub fn get_instance_with_fallback_to_default(
        &self,
        instance_id: Option<&OpenTalkInstanceId>,
    ) -> Option<(OpenTalkInstanceId, OpenTalkInstanceConfig)> {
        if let Some(instance_id) = instance_id {
            let instance = self.get_instance(instance_id)?;
            Some((instance_id.clone(), instance.clone()))
        } else {
            self.get_default_instance()
        }
    }

    /// Get an account config by optional ids, returning the default config at each level if the ids are [None].
    pub fn get_account_with_fallback_to_default(
        &self,
        instance_id: Option<&OpenTalkInstanceId>,
        account_id: Option<&OpenTalkAccountId>,
    ) -> Option<(OpenTalkInstanceAccountId, OpenTalkAccountConfig)> {
        let (instance_id, instance) = self.get_instance_with_fallback_to_default(instance_id)?;
        let (account_id, account) = instance.get_account_with_fallback_to_default(account_id)?;
        Some(((instance_id, account_id).into(), account))
    }

    /// Remove an account from the configuration.
    pub fn remove_account(
        &mut self,
        instance_id: Option<&OpenTalkInstanceId>,
        account_id: Option<&OpenTalkAccountId>,
    ) {
        let Some(instance_id) = self.default_instance.as_ref().or(instance_id) else {
            return;
        };

        let Some(instance) = self.instances.get_mut(instance_id) else {
            return;
        };

        instance.remove_account(account_id);
    }
}
