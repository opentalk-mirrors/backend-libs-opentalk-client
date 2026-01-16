// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Data types representing the responses from the OpenTalk API

mod api_error;
mod api_response;

pub use api_error::ApiError;
pub use api_response::ApiResponse;

/// A result returned from the API.
///
/// This type is intended to be read from a response that was successfully
/// retrieved from an HTTP request. Successful only in the regard that the
/// communication with the HTTP server was successful. The successfully
/// retrieved response can still communicate an error, this will be represented
/// as `Err(ApiError)`, e.g.  when the HTTP status code indicates failure.
pub type ApiResult<T, E = ApiError> = Result<T, E>;
