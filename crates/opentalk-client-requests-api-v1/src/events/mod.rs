// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Requests for the API endpoints under `/events`

mod events_get_by_id_request;
mod events_get_request;
mod events_invites_post_request;
mod events_post_request;

pub use events_get_by_id_request::EventsGetByIdRequest;
pub use events_get_request::EventsGetRequest;
pub use events_invites_post_request::EventsInvitesPostRequest;
pub use events_post_request::EventsPostRequest;
