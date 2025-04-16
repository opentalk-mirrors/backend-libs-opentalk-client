// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Requests for the API endpoints under `/users`

mod users_get_find_query_request;
mod users_get_me_assets;
mod users_get_me_request;
mod users_patch_me_request;

pub use users_get_find_query_request::UsersFindGetRequest;
pub use users_get_me_assets::UsersMeAssetsGetRequest;
pub use users_get_me_request::UsersMeGetRequest;
pub use users_patch_me_request::UsersMePatchRequest;
