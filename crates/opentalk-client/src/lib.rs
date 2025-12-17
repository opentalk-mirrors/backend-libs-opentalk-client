// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! OpenTalk client library

#![deny(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    overflowing_literals,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod authenticated_client;
mod authorization;
mod client;
mod oidc;
mod oidc_authorization;

pub use authenticated_client::AuthenticatedClient;
pub use authorization::Authorization;
pub use client::Client;
pub use oidc_authorization::{OidcDeviceAuthorization, OidcDirectAccessGrant};
