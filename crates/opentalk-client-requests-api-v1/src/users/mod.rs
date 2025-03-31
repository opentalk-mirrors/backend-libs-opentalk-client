// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Requests for the API endpoints under `/users`

mod users_get_find_query_request;
mod users_get_me_request;

pub use users_get_find_query_request::UsersFindGetRequest;
pub use users_get_me_request::UsersMeGetRequest;
