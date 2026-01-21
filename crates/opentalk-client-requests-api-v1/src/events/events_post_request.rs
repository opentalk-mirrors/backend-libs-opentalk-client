// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::events::{EventOptionsQuery, EventResource, PostEventsBody};

use crate::response::ApiResponse;

type Response = ApiResponse<EventResource>;

/// *POST* request on `/events`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
        method = "POST",
        response = Response,
        path = "/events",
)]
pub struct EventsPostRequest {
    /// The query parameters for the request.
    #[http_request(query)]
    pub query: EventOptionsQuery,

    /// The body for the request.
    #[http_request(body)]
    pub body: PostEventsBody,
}
