// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct OidcEndpoints {
    pub device_authorization_endpoint: Url,
    pub authorization_endpoint: Url,
    pub token_endpoint: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, HttpRequest)]
#[http_request(
    method = "GET",
    response = OidcEndpoints,
    path = ".well-known/openid-configuration"
)]
pub(crate) struct OidcWellKnownRequest;
