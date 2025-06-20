// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use anyhow::Result;
use http_request_derive::HttpRequest;
use http_request_derive_client::Client as _;
use http_request_derive_client_reqwest::{ReqwestClient, ReqwestClientError};
use opentalk_client_requests_api_v1::auth::LoginGetRequest;
use opentalk_types_api_v1::auth::{GetLoginResponseBody, OidcProvider};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt as _, Snafu};
use url::Url;

use crate::oidc::{OidcEndpoints, OidcWellKnownRequest};

#[derive(Debug, Snafu)]
pub enum ClientError {
    HttpRequestDeriveClient { source: ReqwestClientError },
}

/// A client for interfacing with the OpenTalk API.
#[derive(Debug)]
pub struct Client {
    inner: ReqwestClient,
    _oidc_endpoints: OidcEndpoints,
}

impl Client {
    /// Discover the OpenTalk API information based on the frontend URL.
    pub async fn discover(url: Url) -> Result<Self> {
        let discovery_client = ReqwestClient::new(url);
        let ClientWellKnownBody {
            opentalk_controller: ControllerBaseInfo { mut base_url },
        } = discovery_client
            .execute(WellKnownRequest)
            .await
            .context(HttpRequestDeriveClientSnafu)?;

        _ = base_url.path_segments_mut().unwrap().push("v1");
        let inner = ReqwestClient::new(base_url);

        let GetLoginResponseBody { oidc } = inner.execute(LoginGetRequest).await?;

        let oidc_url = oidc.url.parse()?;
        let oidc_client = ReqwestClient::new(oidc_url);

        let oidc_endpoints = oidc_client.execute(OidcWellKnownRequest).await?;
        Ok(Self {
            inner,
            _oidc_endpoints: oidc_endpoints,
        })
    }

    /// Query the OIDC provider information from the OpenTalk API
    pub async fn get_oidc_provider(&self) -> Result<OidcProvider> {
        let GetLoginResponseBody { oidc } = self.inner.execute(LoginGetRequest).await?;
        Ok(oidc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, HttpRequest)]
#[http_request(method="GET",response = ClientWellKnownBody,path=".well-known/opentalk/client")]
struct WellKnownRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ControllerBaseInfo {
    base_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ClientWellKnownBody {
    opentalk_controller: ControllerBaseInfo,
}
