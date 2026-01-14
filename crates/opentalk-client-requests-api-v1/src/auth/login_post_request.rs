// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::auth::{PostLoginResponseBody, login::AuthLoginPostRequestBody};

/// *POST* request on `/auth/login`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
    method = "POST",
    response = PostLoginResponseBody,
    path = "/auth/login"
)]
pub struct LoginPostRequest {
    /// Request body
    #[http_request(body)]
    pub body: AuthLoginPostRequestBody,
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use httpmock::MockServer;
    use opentalk_types_api_v1::auth::{PostLoginResponseBody, login::AuthLoginPostRequestBody};
    use pretty_assertions::{assert_eq, assert_matches};

    use crate::{
        auth::login_post_request::LoginPostRequest,
        test_client::{TestClient, TestClientError},
    };

    #[tokio::test]
    async fn successs() {
        let server = MockServer::start();
        let response_body = PostLoginResponseBody {
            permissions: BTreeSet::default(),
        };

        let request_body = AuthLoginPostRequestBody {
            id_token: "spoijayxdoindsfoikshdfojhslkdfhoisdhf".to_string(),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("POST")
                .path("/auth/login")
                .json_body_obj(&request_body);
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(&server, LoginPostRequest { body: request_body })
            .await
            .expect("valid answer expected");

        mock.assert();

        assert_eq!(response_body, response);
    }

    #[tokio::test]
    async fn internal_server_error() {
        let server = MockServer::start();
        let response_body = PostLoginResponseBody {
            permissions: BTreeSet::default(),
        };

        let request_body = AuthLoginPostRequestBody {
            id_token: "spoijayxdoindsfoikshdfojhslkdfhoisdhf".to_string(),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("POST")
                .path("/auth/login")
                .json_body_obj(&request_body);
            _ = then.status(500).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, LoginPostRequest { body: request_body })
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::NonSuccessStatus {
                    status: http::StatusCode::INTERNAL_SERVER_ERROR,
                    body: _,
                },
                message: _,
            })
        );

        mock.assert();
    }

    #[tokio::test]
    async fn bad_request_error() {
        let server = MockServer::start();
        let response_body = PostLoginResponseBody {
            permissions: BTreeSet::default(),
        };

        let request_body = AuthLoginPostRequestBody {
            id_token: "spoijayxdoindsfoikshdfojhslkdfhoisdhf".to_string(),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("POST")
                .path("/auth/login")
                .json_body_obj(&request_body);
            _ = then.status(400).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, LoginPostRequest { body: request_body })
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::NonSuccessStatus {
                    status: http::StatusCode::BAD_REQUEST,
                    body: _,
                },
                message: _,
            })
        );

        mock.assert();
    }

    #[tokio::test]
    async fn unauthorized_error() {
        let server = MockServer::start();
        let response_body = PostLoginResponseBody {
            permissions: BTreeSet::default(),
        };

        let request_body = AuthLoginPostRequestBody {
            id_token: "spoijayxdoindsfoikshdfojhslkdfhoisdhf".to_string(),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("POST")
                .path("/auth/login")
                .json_body_obj(&request_body);
            _ = then.status(401).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, LoginPostRequest { body: request_body })
                .await,
            Err(TestClientError::HttpRequestDerive {
                source,
                message: _,
            }) if source.is_unauthorized()
        );

        mock.assert();
    }
}
