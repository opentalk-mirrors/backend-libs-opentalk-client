// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use http_request_derive::HttpRequest;
use httpmock::MockServer;
use reqwest::Client;
use snafu::{ResultExt as _, Snafu};

#[derive(Debug, Snafu)]
pub enum TestClientError {
    HttpRequestDerive {
        source: http_request_derive::Error,
        message: String,
    },
    Reqwest {
        source: reqwest::Error,
        message: String,
    },
    Http {
        source: http::Error,
        message: String,
    },
}

pub(crate) struct TestClient {
    http_client: Client,
}

impl TestClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub async fn send_request<R: HttpRequest + Send>(
        &self,
        mock_server: &MockServer,
        request: R,
    ) -> Result<R::Response, TestClientError> {
        let base_url = mock_server.base_url().parse().unwrap();
        let request = request
            .to_http_request(&base_url)
            .context(HttpRequestDeriveSnafu {
                message: "failed to create http request from base url".to_string(),
            })?;
        let request = reqwest::Request::try_from(request).context(ReqwestSnafu {
            message: "failed to convert http request into reqwest request".to_string(),
        })?;
        let response = self
            .http_client
            .execute(request)
            .await
            .context(ReqwestSnafu {
                message: "failed to execute http request".to_string(),
            })?;
        let mut http_response = http::Response::builder()
            .status(response.status())
            .version(response.version());
        if let Some(headers) = http_response.headers_mut() {
            *headers = response.headers().clone();
        }
        let body = response.bytes().await.context(ReqwestSnafu {
            message: "failed to read http response bytes",
        })?;
        let http_response = http_response.body(body).context(HttpSnafu {
            message: "failed to set body for http response".to_string(),
        })?;
        R::read_response(http_response).context(HttpRequestDeriveSnafu {
            message: "failed to read http request derive response from http response".to_string(),
        })
    }
}
