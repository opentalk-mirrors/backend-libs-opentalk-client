// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::events::{PostEventInviteBody, PostEventInviteQuery};
use opentalk_types_common::events::EventId;

use crate::{common::EmptyBody, response::ApiResponse};

type Response = ApiResponse<EmptyBody>;

/// *POST* request on `/events/{event_id}/invites`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
        method = "POST",
        response = Response,
        path = "/events/{event_id}/invites",
)]
pub struct EventsInvitesPostRequest {
    /// The id of the event to which the participant should be invited.
    pub event_id: EventId,

    /// The query parameters for the request.
    #[http_request(query)]
    pub query: PostEventInviteQuery,

    /// The body for the request.
    #[http_request(body)]
    pub body: PostEventInviteBody,
}
