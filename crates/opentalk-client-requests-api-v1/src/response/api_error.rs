// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use bytes::Bytes;
use http::StatusCode;
use snafu::Snafu;

/// An error returned from an API request
#[derive(Debug, Snafu)]
pub enum ApiError {
    /// The API returned an HTTP 401 UNAUTHORIZED status
    #[snafu(display("The API returned an HTTP 401 UNAUTHORIZED status"))]
    Unauthorized {
        /// The headers sent with the response
        headers: BTreeMap<String, String>,

        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 403 FORBIDDEN status
    #[snafu(display("The API returned an HTTP 403 FORBIDDEN status"))]
    Forbidden {
        /// The headers sent with the response
        headers: BTreeMap<String, String>,

        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 404 NOT FOUND status
    #[snafu(display("The API returned an HTTP 404 NOT FOUND status"))]
    NotFound {
        /// The headers sent with the response
        headers: BTreeMap<String, String>,

        /// The body of the response
        body: Bytes,
    },

    /// The API returned an HTTP 500 INTERNAL SERVER ERROR status
    #[snafu(display("The API returned an HTTP 500 INTERNAL SERVER ERROR status"))]
    InternalServerError {
        /// The headers sent with the response
        headers: BTreeMap<String, String>,

        /// The body of the response
        body: Bytes,
    },

    /// The API returned an other non-successful HTTP status code
    #[snafu(display("Received non-successful HTTP status code {code}"))]
    NonSuccessfulStatusCode {
        /// The headers sent with the response
        headers: BTreeMap<String, String>,

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
        let headers = http_response
            .headers()
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    String::from_utf8_lossy(v.as_bytes()).to_string(),
                )
            })
            .collect();

        match status {
            StatusCode::UNAUTHORIZED => Ok(Self::Unauthorized {
                headers,
                body: http_response.into_body(),
            }),
            StatusCode::FORBIDDEN => Ok(Self::Forbidden {
                headers,
                body: http_response.into_body(),
            }),
            StatusCode::NOT_FOUND => Ok(Self::NotFound {
                headers,
                body: http_response.into_body(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR => Ok(Self::InternalServerError {
                headers,
                body: http_response.into_body(),
            }),
            code if !code.is_success() => Ok(Self::NonSuccessfulStatusCode {
                headers,
                code,
                body: http_response.into_body(),
            }),
            _ => Err(http_response),
        }
    }

    /// Query whether the error is caused by a specific HTTP status code.
    pub const fn is_http_status_code(&self, expected_code: StatusCode) -> bool {
        let c = expected_code.as_u16();
        match self {
            ApiError::Unauthorized { .. } => c == StatusCode::UNAUTHORIZED.as_u16(),
            ApiError::Forbidden { .. } => c == StatusCode::FORBIDDEN.as_u16(),
            ApiError::NotFound { .. } => c == StatusCode::NOT_FOUND.as_u16(),
            ApiError::InternalServerError { .. } => c == StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ApiError::NonSuccessfulStatusCode { code, .. } => c == code.as_u16(),
        }
    }

    /// Query whether the error is caused by a 401 UNAUTHORIZED HTTP status code
    pub const fn is_unauthorized(&self) -> bool {
        self.is_http_status_code(StatusCode::UNAUTHORIZED)
    }

    /// Query whether the error is caused by a 403 FORBIDDEN HTTP status code
    pub const fn is_forbidden(&self) -> bool {
        self.is_http_status_code(StatusCode::FORBIDDEN)
    }

    /// Query whether the error is caused by a 404 NOT FOUND HTTP status code
    pub const fn is_not_found(&self) -> bool {
        self.is_http_status_code(StatusCode::NOT_FOUND)
    }

    /// Query whether the error is caused by a 500 INTERNAL SERVER ERROR HTTP status code
    pub const fn is_internal_server_error(&self) -> bool {
        self.is_http_status_code(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
