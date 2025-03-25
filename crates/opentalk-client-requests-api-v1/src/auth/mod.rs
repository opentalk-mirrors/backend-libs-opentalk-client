// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Requests for the API endpoints under `/auth`

mod login_get_request;
mod login_post_request;

pub use login_get_request::LoginGetRequest;
pub use login_post_request::LoginPostRequest;
