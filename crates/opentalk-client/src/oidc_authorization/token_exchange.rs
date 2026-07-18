// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::{HttpRequest, HttpRequestBody};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(HttpRequest)]
#[http_request(method = "POST", response = TokenExchangeResponse, path = "")]
pub struct TokenExchangeRequest {
    #[http_request(body)]
    pub body: TokenExchangeRequestBody,
}

#[derive(Debug, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TokenExchangeRequestBody {
    pub client_id: String,
    pub subject_token: SecretString,
}

impl HttpRequestBody for TokenExchangeRequestBody {
    fn to_vec(&self) -> Result<Vec<u8>, http_request_derive::Error> {
        let encoded = url::form_urlencoded::Serializer::new(String::new())
            .append_pair(
                "grant_type",
                "urn:ietf:params:oauth:grant-type:token-exchange",
            )
            .append_pair("client_id", &self.client_id)
            .append_pair("subject_token", self.subject_token.expose_secret())
            .append_pair(
                "subject_token_type",
                "urn:ietf:params:oauth:token-type:access_token",
            )
            .append_pair("audience", &self.client_id)
            .append_pair("scope", "openid profile email")
            .finish();
        Ok(encoded.into_bytes())
    }

    fn apply_headers(&self, headers: &mut http::HeaderMap) {
        let _ =
            headers
                .entry(http::header::CONTENT_TYPE)
                .or_insert(http::HeaderValue::from_static(
                    "application/x-www-form-urlencoded",
                ));
    }
}
