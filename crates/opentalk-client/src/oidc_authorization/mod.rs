// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

mod oidc_device_authorization;
mod oidc_direct_access_grant;

pub use oidc_device_authorization::OidcDeviceAuthorization;
pub use oidc_direct_access_grant::OidcDirectAccessGrant;

#[allow(unused)]
const REFRESH_BEFORE_EXPIRY: Duration = Duration::from_secs(10);
