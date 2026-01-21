// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use http_request_derive::FromHttpResponse;

/// An empty body response
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyBody;

impl FromHttpResponse for EmptyBody {
    fn from_http_response(
        http_response: http::Response<Bytes>,
    ) -> Result<Self, http_request_derive::Error>
    where
        Self: Sized,
    {
        let status = http_response.status();

        if !status.is_success() {
            return Err(http_request_derive::Error::NonSuccessStatus {
                status,
                body: http_response.into_body(),
            });
        }

        Ok(Self)
    }
}
