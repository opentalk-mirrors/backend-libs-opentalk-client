// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use http::StatusCode;
use snafu::Snafu;

/// An error returned from an API request
#[derive(Debug, Snafu)]
pub enum ApiError {
    /// The API returned an HTTP 401 UNAUTHORIZED status
    Unauthorized {
        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 403 FORBIDDEN status
    Forbidden {
        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 404 NOT FOUND status
    NotFound {
        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 500 INTERNAL SERVER ERROR status
    InternalServerError {
        /// The body of the response
        body: Bytes,
    },

    /// The API returned an other non-successful HTTP status code
    #[snafu(display("Received non-successful HTTP status code {code}"))]
    NonSuccessfulStatusCode {
        /// The non-success status code returned in the response
        code: StatusCode,
        /// The body of the response
        body: Bytes,
    },
}

impl ApiError {
    /// Build an [`ApiError`] from the response if it is considered non-successful.
    /// Returns an [`Err`] with the response if it is considered successful, can be used for further processing.
    #[allow(clippy::result_large_err)]
    pub fn try_read_from_http_response(
        http_response: http::Response<Bytes>,
    ) -> Result<Self, http::Response<Bytes>> {
        let status = http_response.status();

        match status {
            StatusCode::UNAUTHORIZED => Ok(Self::Unauthorized {
                body: http_response.into_body(),
            }),
            StatusCode::FORBIDDEN => Ok(Self::Forbidden {
                body: http_response.into_body(),
            }),
            StatusCode::NOT_FOUND => Ok(Self::NotFound {
                body: http_response.into_body(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR => Ok(Self::InternalServerError {
                body: http_response.into_body(),
            }),
            code if !code.is_success() => Ok(Self::NonSuccessfulStatusCode {
                code,
                body: http_response.into_body(),
            }),
            _ => Err(http_response),
        }
    }
}
