// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::events::{EventResource, GetEventQuery};
use opentalk_types_common::events::EventId;

use crate::response::ApiResponse;

type Response = ApiResponse<EventResource>;

/// *GET* request on `/events/{event_id}`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
        method = "GET",
        response = Response,
        path = "/events/{event_id}",
)]
pub struct EventsGetByIdRequest {
    /// The id of the event to which the participant should be invited.
    pub event_id: EventId,

    /// The query parameters for the request.
    #[http_request(query)]
    pub query: GetEventQuery,
}
