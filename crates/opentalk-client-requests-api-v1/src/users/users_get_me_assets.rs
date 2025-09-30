// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use opentalk_types_api_v1::users::GetUserAssetsResponseBody;
use opentalk_types_common::{assets::AssetSorting, order::Ordering};

/// *GET* request on `/users/me/assets`
#[derive(Clone, Debug, PartialEq, Eq, HttpRequest)]
#[http_request(
        method = "GET",
        response = GetUserAssetsResponseBody,
        path = "/users/me/assets",
)]
pub struct UsersMeAssetsGetRequest {
    /// The number of entries per page
    pub per_page: i64,
    /// The number of the page
    pub page: i64,
    /// The optional sorting query parameter
    pub sort: Option<AssetSorting>,
    /// The sorting order that should be applied to the collection
    pub order: Ordering,
}
#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use httpmock::MockServer;
    use opentalk_types_api_v1::{
        assets::AssetResource,
        users::{GetUserAssetsResponseBody, UserAssetResource},
    };
    use opentalk_types_common::{
        assets::{AssetId, AssetSorting},
        events::EventId,
        modules::ModuleId,
        order::Ordering,
        rooms::RoomId,
        utils::ExampleData,
    };
    use pretty_assertions::assert_matches;

    use crate::{
        test_client::{TestClient, TestClientError},
        users::users_get_me_assets::UsersMeAssetsGetRequest,
    };

    #[tokio::test]
    async fn success() {
        let server = MockServer::start();
        let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap();
        let response_body = GetUserAssetsResponseBody {
            owned_assets: vec![UserAssetResource {
                asset: AssetResource {
                    id: AssetId::example_data(),
                    filename: "qwertz.png".to_string(),
                    namespace: Some(ModuleId::example_data()),
                    created_at: dt,
                    kind: "".to_string(),
                    size: 10.into(),
                },
                room_id: RoomId::example_data(),
                event_id: Some(EventId::example_data()),
            }],
        };

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me/assets");
            _ = then.status(200).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        let response = client
            .send_request(
                &server,
                UsersMeAssetsGetRequest {
                    per_page: 50,
                    page: 1,
                    sort: Some(AssetSorting::Namespace),
                    order: Ordering::Ascending,
                },
            )
            .await
            .expect("valid answer expected");

        mock.assert();
        assert_eq!(response_body, response);
    }

    #[tokio::test]
    async fn internal_server_error() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me/assets");
            _ = then.status(500).body("internal server error");
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(
                    &server,
                    UsersMeAssetsGetRequest {
                        per_page: 50,
                        page: 1,
                        sort: Some(AssetSorting::Namespace),
                        order: Ordering::Ascending,
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
        let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap();
        let response_body = GetUserAssetsResponseBody {
            owned_assets: vec![UserAssetResource {
                asset: AssetResource {
                    id: AssetId::example_data(),
                    filename: "qwertz.png".to_string(),
                    namespace: Some(ModuleId::example_data()),
                    created_at: dt,
                    kind: "".to_string(),
                    size: 10.into(),
                },
                room_id: RoomId::example_data(),
                event_id: Some(EventId::example_data()),
            }],
        };

        let mock = server.mock(|when, then| {
            _ = when.method("GET").path("/users/me/assets");
            _ = then.status(401).json_body_obj(&response_body);
        });

        let client = TestClient::new();

        assert_matches!(
            client
                .send_request(
                    &server,
                    UsersMeAssetsGetRequest {
                        per_page: 50,
                        page: 1,
                        sort: Some(AssetSorting::Namespace),
                        order: Ordering::Ascending,
                    },
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
