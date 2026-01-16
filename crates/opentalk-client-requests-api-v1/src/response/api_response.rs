// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use http_request_derive::FromHttpResponse;

use crate::response::{ApiError, ApiResult};

/// A response that can be read using the [`http_request_derive::FromHttpResponse`] trait.
#[derive(Debug)]
pub struct ApiResponse<T>(pub ApiResult<T>);

impl<T: FromHttpResponse> FromHttpResponse for ApiResponse<T> {
    fn from_http_response(
        http_response: http::Response<Bytes>,
    ) -> Result<Self, http_request_derive::Error>
    where
        Self: Sized,
    {
        let http_response = {
            match ApiError::try_read_from_http_response(http_response) {
                Ok(e) => return Ok(ApiResponse(Err(e))),
                Err(response) => response,
            }
        };
        let response = T::from_http_response(http_response)?;
        Ok(ApiResponse(Ok(response)))
    }
}

impl<T> ApiResponse<T> {
    /// Extract the contained [`ApiResult`] from the [`ApiResponse`].
    pub fn into_inner(self) -> ApiResult<T> {
        self.0
    }
}
