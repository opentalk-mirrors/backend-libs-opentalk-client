// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::From,
    derive_more::FromStr,
)]
pub struct OpenTalkInstanceId(Url);

impl OpenTalkInstanceId {
    pub fn to_file_name(&self) -> String {
        format!(
            "{}_{}",
            self.0.host_str().unwrap(),
            self.0.path().replace("_", "__").replace("/", "_")
        )
    }
}
