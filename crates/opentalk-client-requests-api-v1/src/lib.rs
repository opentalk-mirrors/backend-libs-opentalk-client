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

pub mod auth;
pub mod events;
pub mod response;
pub mod users;

pub use events::{EventsGetRequest, EventsInvitesPostRequest, EventsPostRequest};
pub use users::{
    UsersFindGetRequest, UsersMeAssetsGetRequest, UsersMeGetRequest, UsersMePatchRequest,
};

pub mod common;

#[cfg(test)]
mod test_client;
