// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::users::{PrivateUserProfile, me::PatchMeRequestBody};

/// *PATCH* request on `/users/me`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
    method = "PATCH",
    response = PrivateUserProfile,
    path = "/users/me"
)]
pub struct UsersMePatchRequest {
    /// Request body
    #[http_request(body)]
    pub body: PatchMeRequestBody,
}

#[cfg(test)]
mod test {
    use httpmock::MockServer;
    use opentalk_types_api_v1::users::{PrivateUserProfile, me::PatchMeRequestBody};
    use opentalk_types_common::{
        tariffs::TariffStatus,
        users::{DisplayName, Language, Theme, UserId, UserTitle},
        utils::ExampleData,
    };
    use pretty_assertions::assert_matches;

    use crate::{
        test_client::{TestClient, TestClientError},
        users::users_patch_me_request::UsersMePatchRequest,
    };

    #[tokio::test]
    async fn successs() {
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

        let request_body = PatchMeRequestBody {
            title: Some(UserTitle::example_data()),
            display_name: Some(DisplayName::example_data()),
            language: Some(Language::example_data()),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("PATCH")
                .path("/users/me")
                .json_body_obj(&request_body);
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(&server, UsersMePatchRequest { body: request_body })
            .await
            .expect("valid answer expected");

        mock.assert();

        assert_eq!(response_body.id, response.id);
    }

    #[tokio::test]
    async fn internal_server_error() {
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

        let request_body = PatchMeRequestBody {
            title: Some(UserTitle::example_data()),
            display_name: Some(DisplayName::example_data()),
            language: Some(Language::example_data()),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("PATCH")
                .path("/users/me")
                .json_body_obj(&request_body);
            _ = then.status(500).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, UsersMePatchRequest { body: request_body })
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
    async fn bad_request_error() {
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

        let request_body = PatchMeRequestBody {
            title: Some(UserTitle::example_data()),
            display_name: Some(DisplayName::example_data()),
            language: Some(Language::example_data()),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("PATCH")
                .path("/users/me")
                .json_body_obj(&request_body);
            _ = then.status(400).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, UsersMePatchRequest { body: request_body })
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::NonSuccessStatus {
                    status: http::StatusCode::BAD_REQUEST,
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

        let request_body = PatchMeRequestBody {
            title: Some(UserTitle::example_data()),
            display_name: Some(DisplayName::example_data()),
            language: Some(Language::example_data()),
            dashboard_theme: Some(Theme::example_data()),
            conference_theme: Some(Theme::example_data()),
        };

        let mock = server.mock(|when, then| {
            _ = when
                .method("PATCH")
                .path("/users/me")
                .json_body_obj(&request_body);
            _ = then.status(401).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(&server, UsersMePatchRequest { body: request_body })
                .await,
            Err(TestClientError::HttpRequestDerive {
                source: http_request_derive::Error::Unauthorized,
                message: _,
            })
        );

        mock.assert();
    }
}
