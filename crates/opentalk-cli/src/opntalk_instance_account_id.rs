// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{opentalk_account_id::OpenTalkAccountId, opentalk_instance_id::OpenTalkInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpenTalkInstanceAccountId(OpenTalkInstanceId, OpenTalkAccountId);

impl From<(OpenTalkInstanceId, OpenTalkAccountId)> for OpenTalkInstanceAccountId {
    fn from((instance, account): (OpenTalkInstanceId, OpenTalkAccountId)) -> Self {
        Self(instance, account)
    }
}

impl OpenTalkInstanceAccountId {
    pub fn to_file_stem(&self) -> String {
        format!("{}_{}", self.0.to_file_name(), *self.1)
    }
}
