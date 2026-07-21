// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

mod oauth_reqwest_0_13_client;
mod oidc_device_authorization;
mod oidc_direct_access_grant;
mod oidc_grant;
mod oidc_token_exchange_grant;
mod token_exchange;

use oauth_reqwest_0_13_client::ClientWrapper;
pub use oidc_device_authorization::OidcDeviceAuthorization;
pub use oidc_direct_access_grant::OidcDirectAccessGrant;
pub use oidc_grant::OidcGrant;
pub use oidc_token_exchange_grant::OidcTokenExchangeGrant;
pub(super) use token_exchange::{TokenExchangeRequest, TokenExchangeRequestBody};

#[allow(unused)]
const REFRESH_BEFORE_EXPIRY: Duration = Duration::from_secs(10);
