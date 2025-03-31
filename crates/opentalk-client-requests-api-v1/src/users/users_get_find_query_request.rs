// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::users::GetFindResponseBody;

/// *GET* request on `/users/find`
#[derive(Clone, Debug, PartialEq, Eq, Hash, HttpRequest)]
#[http_request(
        method = "GET",
        response = GetFindResponseBody,
        path = "/users/find",
)]
pub struct UsersFindGetRequest {
    /// Query string
    #[http_request(body)]
    pub q: String,
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use opentalk_types_api_v1::users::{GetFindResponseBody, GetFindResponseEntry};
    use pretty_assertions::assert_matches;

    use crate::{
        test_client::{TestClient, TestClientError},
        users::users_get_find_query_request::UsersFindGetRequest,
    };

    #[tokio::test]
    async fn success() {
        let server = MockServer::start();

        let users: Vec<GetFindResponseEntry> = Vec::new();
        let response_body = GetFindResponseBody(users);

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/find");
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(
                &server,
                UsersFindGetRequest {
                    q: "bob".to_string(),
                },
            )
            .await
            .expect("valid answer expected");

        mock.assert();

        assert_eq!(response_body, response)
    }

    #[tokio::test]
    async fn internal_server_error() {
        let server = MockServer::start();

        let users: Vec<GetFindResponseEntry> = Vec::new();
        let response_body = GetFindResponseBody(users);

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/find");
            _ = then.status(500).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(
                    &server,
                    UsersFindGetRequest {
                        q: "bob".to_string()
                    }
                )
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::NonSuccessStatus {
                    status: http::StatusCode::INTERNAL_SERVER_ERROR,
                    data: _,
                },
                message: _,
            })
        );

        mock.assert();
    }

    #[tokio::test]
    async fn unauthorized_error() {
        let server = MockServer::start();

        let users: Vec<GetFindResponseEntry> = Vec::new();
        let response_body = GetFindResponseBody(users);

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/find");
            _ = then.status(401).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(
                    &server,
                    UsersFindGetRequest {
                        q: "bob".to_string()
                    }
                )
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::Unauthorized,
                message: _,
            })
        );

        mock.assert();
    }
}
