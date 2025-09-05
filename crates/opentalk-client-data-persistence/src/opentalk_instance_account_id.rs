// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{opentalk_account_id::OpenTalkAccountId, opentalk_instance_id::OpenTalkInstanceId};

/// The full id of a stored OpenTalk account, including the [OpenTalkInstanceId] and the [OpenTalkAccountId].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpenTalkInstanceAccountId(OpenTalkInstanceId, OpenTalkAccountId);

impl OpenTalkInstanceAccountId {
    /// Get the instance id.
    pub fn instance_id(&self) -> &OpenTalkInstanceId {
        &self.0
    }

    /// Get the account id.
    pub fn account_id(&self) -> &OpenTalkAccountId {
        &self.1
    }
}

impl From<(OpenTalkInstanceId, OpenTalkAccountId)> for OpenTalkInstanceAccountId {
    fn from((instance, account): (OpenTalkInstanceId, OpenTalkAccountId)) -> Self {
        Self(instance, account)
    }
}

impl OpenTalkInstanceAccountId {
    /// Convert the id to a file stem that can be used to store the account data.
    pub fn to_file_stem(&self) -> String {
        format!("{}_{}", self.0.to_file_name(), *self.1)
    }
}
