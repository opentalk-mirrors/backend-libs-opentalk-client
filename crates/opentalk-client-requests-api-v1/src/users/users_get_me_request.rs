// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::users::PrivateUserProfile;

/// *GET* request on `/users/me`
#[derive(Clone, Debug, PartialEq, Eq, Hash, HttpRequest)]
#[http_request(
        method = "GET",
        response = PrivateUserProfile,
        path = "/users/me",
)]
pub struct UsersMeGetRequest;

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use opentalk_types_api_v1::users::PrivateUserProfile;
    use opentalk_types_common::{
        tariffs::TariffStatus,
        users::{DisplayName, Language, Theme, UserId, UserTitle},
        utils::ExampleData,
    };
    use pretty_assertions::assert_matches;

    use crate::{
        test_client::{TestClient, TestClientError},
        users::users_get_me_request::UsersMeGetRequest,
    };

    #[tokio::test]
    async fn success() {
        let server = MockServer::start();
        let response_body = PrivateUserProfile {
            id: UserId::example_data(),
            email: "mail@example.com".to_string(),
            title: UserTitle::example_data(),
            firstname: "Bob".to_string(),
            lastname: "Müller".to_string(),
            display_name: DisplayName::example_data(),
            avatar_url: "".to_string(),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
            tariff_status: TariffStatus::Default,
            language: Language::default(),
            used_storage: 0,
        };

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me");
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(&server, UsersMeGetRequest)
            .await
            .expect("valid answer expected");

        mock.assert();

        assert_eq!(response_body.id, response.id);
    }

    #[tokio::test]
    async fn internal_server_error() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me");
            _ = then.status(500).body("internal server error");
        });

        let client = TestClient::new();

        assert_matches!(
            client.send_request(&server, UsersMeGetRequest).await,
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

        let response_body = PrivateUserProfile {
            id: UserId::example_data(),
            email: "mail@example.com".to_string(),
            title: UserTitle::example_data(),
            firstname: "Bob".to_string(),
            lastname: "Müller".to_string(),
            display_name: DisplayName::example_data(),
            avatar_url: "".to_string(),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
            tariff_status: TariffStatus::Default,
            language: Language::default(),
            used_storage: 0,
        };

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me");
            _ = then.status(401).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client.send_request(&server, UsersMeGetRequest {}).await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::Unauthorized,
                message: _,
            })
        );

        mock.assert();
    }
}
