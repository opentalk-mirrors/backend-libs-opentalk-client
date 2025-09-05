// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// A trait for providing authorization tokens for client requests.
#[async_trait::async_trait]
pub trait Authorization {
    /// Get a valid access token.
    ///
    /// The implementation of this method may perform extra actions under the
    /// hood, e.g. refreshing expired tokens.
    async fn get_access_token(&self) -> anyhow::Result<String>;
}
