// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use secrecy::SecretString;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
/// OAuth2/OIDC endpoints as pers [spec](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata)
pub struct OidcEndpoints {
    /// [Device Authorization Endpoint](https://datatracker.ietf.org/doc/html/rfc8628#section-3.1)
    pub device_authorization_endpoint: Option<Url>,
    /// [Authorization Endpoint](https://openid.net/specs/openid-connect-core-1_0.html#AuthorizationEndpoint)
    pub authorization_endpoint: Url,
    /// [Token Endpoint](https://openid.net/specs/openid-connect-core-1_0.html#TokenEndpoint)
    pub token_endpoint: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, HttpRequest)]
#[http_request(
    method = "GET",
    response = OidcEndpoints,
    path = ".well-known/openid-configuration"
)]
pub(crate) struct OidcWellKnownRequest;

/// Supported auth methods to obtain OAuth2/OIDC grants
#[derive(Debug)]
pub enum OidcAuthMethod {
    /// Used to obtain a Resource Owner Password (Direct Access) grant
    DirectAccess {
        /// Username
        username: String,
        /// Password
        password: SecretString,
    },
    /// Used to obtain a Token Exchange grant
    TokenExchange {
        /// Token, that will be exchanged against an OIDC access token
        subject_token: SecretString,
    },
    /// Used to obtain a Device Authorization grant
    DeviceAuthorization,
}
