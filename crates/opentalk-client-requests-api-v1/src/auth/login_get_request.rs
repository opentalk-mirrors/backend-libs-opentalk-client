// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::auth::GetLoginResponseBody;

/// *GET* request on `/auth/login`
#[derive(Clone, Debug, PartialEq, Eq, Hash, HttpRequest)]
#[http_request(
        method = "GET",
        response = GetLoginResponseBody,
        path = "/auth/login",
)]
pub struct LoginGetRequest;

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use opentalk_types_api_v1::auth::{GetLoginResponseBody, OidcProvider};
    use pretty_assertions::assert_matches;

    use crate::{
        auth::LoginGetRequest,
        test_client::{TestClient, TestClientError},
    };

    #[tokio::test]
    async fn success() {
        let server = MockServer::start();

        let response_body = GetLoginResponseBody {
            oidc: OidcProvider {
                name: "Example Auth".to_string(),
                url: "https://auth.example.com/".to_string(),
            },
        };

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/auth/login");
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(&server, LoginGetRequest)
            .await
            .expect("valid answer expected");

        mock.assert();

        assert_eq!(response_body, response);
    }

    #[tokio::test]
    async fn internal_server_error() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/auth/login");
            _ = then.status(500).body("internal server error");
        });

        let client = TestClient::new();

        assert_matches!(
            client.send_request(&server, LoginGetRequest).await,
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
}
