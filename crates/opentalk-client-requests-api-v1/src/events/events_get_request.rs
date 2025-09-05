// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::events::EventOrException;

type Response = Vec<EventOrException>;

/// *GET* request on `/events`
#[derive(Clone, Debug, PartialEq, Eq, Hash, HttpRequest)]
#[http_request(
        method = "GET",
        response = Response,
        path = "/events",
)]
pub struct EventsGetRequest;
