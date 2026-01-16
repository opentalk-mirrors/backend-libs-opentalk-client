// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use http_request_derive::HttpRequest;
use http_request_derive_client::Client as _;
use http_request_derive_client_reqwest::{ReqwestClient, ReqwestClientError};
use itertools::Itertools as _;
use opentalk_client_requests_api_v1::{auth::LoginGetRequest, response::ApiError};
use opentalk_types_api_v1::auth::{GetLoginResponseBody, OidcProvider};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt as _, Snafu, ensure};
use url::Url;

use crate::{
    AuthenticatedClient, Authorization,
    oidc::{OidcEndpoints, OidcWellKnownRequest},
};

const COMPATIBLE_VERSIONS: &[&str] = &["v1"];

#[derive(Debug, Snafu)]
pub enum ClientError {
    #[snafu(display("Reqwest returned an error"))]
    Reqwest { source: ReqwestClientError },

    #[snafu(display("The API server returned an error"))]
    Api { source: ApiError },

    #[snafu(display(
        "No compatible API version found under the well-known API endpoint {url}. This client is compatible with API versions: {compatible_versions}."
    ))]
    NoCompatibleApiVersion {
        url: Url,
        compatible_versions: String,
    },

    #[snafu(display("Invalid OIDC url found: {url:?}"))]
    InvalidOidcUrl {
        url: String,
        source: url::ParseError,
    },

    #[snafu(display(
        "Discovered url {url} which cannot be a base and therefore is not a valid controller API url"
    ))]
    InvalidUrlDiscovered { url: Url },
}

/// A client for interfacing with the OpenTalk API.
#[derive(Debug)]
pub struct Client {
    inner: ReqwestClient,
    #[allow(unused)]
    oidc_url: Url,
    #[allow(unused)]
    api_url: Url,
}

impl Client {
    /// Discover the OpenTalk API information based on the frontend or controller api URL.
    pub async fn discover(url: Url) -> Result<Self, ClientError> {
        let discovery_client = ReqwestClient::new(url.clone());

        match discovery_client
            .execute(WellKnownFrontendRequest)
            .await
            .context(ReqwestSnafu)?
        {
            WellKnownFrontendResponse::Found(WellKnownFrontendBody {
                opentalk_controller: ControllerBaseInfo { base_url },
            }) => Self::discover_controller(base_url).await,
            WellKnownFrontendResponse::NotFound => Self::discover_controller(url).await,
        }
    }

    /// Discover the OpenTalk API information based on the controller api URL.
    pub async fn discover_controller(url: Url) -> Result<Self, ClientError> {
        let discovery_client = ReqwestClient::new(url.clone());

        let WellKnownApiBody {
            opentalk_api: ApiInfo { v1 },
        } = discovery_client
            .execute(WellKnownApiRequest)
            .await
            .context(ReqwestSnafu)?;

        let Some(VersionedApiInfo { base_url }) = v1 else {
            return NoCompatibleApiVersionSnafu {
                url,
                compatible_versions: COMPATIBLE_VERSIONS.iter().join(", "),
            }
            .fail();
        };

        let api_url = match Url::parse(&base_url) {
            Ok(url) => {
                ensure!(!url.cannot_be_a_base(), InvalidUrlDiscoveredSnafu { url });
                url
            }
            Err(_e) => {
                let segments = base_url.trim_start_matches('/');
                let mut url = url;
                _ = url.path_segments_mut().unwrap().push(segments);
                url
            }
        };

        let inner = ReqwestClient::new(api_url.clone());

        let GetLoginResponseBody { oidc } =
            inner.execute(LoginGetRequest).await.context(ReqwestSnafu)?;

        let oidc_url = oidc
            .url
            .parse()
            .context(InvalidOidcUrlSnafu { url: oidc.url })?;

        Ok(Self {
            oidc_url,
            api_url,
            inner,
        })
    }

    /// Get the oidc endpoints from the OIDC provider.
    pub async fn get_oidc_endpoints(&self) -> Result<OidcEndpoints, ClientError> {
        let oidc_client = ReqwestClient::new(self.oidc_url.clone());
        let oidc_endpoints = oidc_client
            .execute(OidcWellKnownRequest)
            .await
            .context(ReqwestSnafu)?;
        Ok(oidc_endpoints)
    }

    /// Query the OIDC provider information from the OpenTalk API
    pub async fn get_oidc_provider(&self) -> Result<OidcProvider, ClientError> {
        let GetLoginResponseBody { oidc } = self
            .inner
            .execute(LoginGetRequest)
            .await
            .context(ReqwestSnafu)?;
        Ok(oidc)
    }

    /// execute request without authorization
    pub async fn execute<R: HttpRequest + Send>(
        &self,
        request: R,
    ) -> Result<R::Response, ReqwestClientError> {
        self.inner.execute(request).await
    }

    /// execute request with authorization
    pub async fn execute_authorized<R: HttpRequest + Send, A: Authorization + Sync>(
        &self,
        request: R,
        authorization: A,
    ) -> Result<R::Response, ReqwestClientError> {
        let authenticated_client = AuthenticatedClient::new(self.inner.clone(), authorization);
        authenticated_client.execute(request).await
    }

    // fn refresh_access_token(&self, instance_account_id: OpenTalkInstanceAccountId)
}

#[derive(Debug, Clone, PartialEq, Eq, HttpRequest)]
#[http_request(method="GET", response = WellKnownFrontendResponse, path=".well-known/opentalk/client")]
struct WellKnownFrontendRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ControllerBaseInfo {
    pub base_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WellKnownFrontendBody {
    pub opentalk_controller: ControllerBaseInfo,
}

enum WellKnownFrontendResponse {
    NotFound,
    Found(WellKnownFrontendBody),
}

impl http_request_derive::FromHttpResponse for WellKnownFrontendResponse {
    fn from_http_response(
        http_response: http::Response<Bytes>,
    ) -> Result<Self, http_request_derive::Error>
    where
        Self: Sized,
    {
        match <WellKnownFrontendBody as http_request_derive::FromHttpResponse>::from_http_response(
            http_response,
        ) {
            Ok(body) => Ok(Self::Found(body)),
            Err(e) if e.is_not_found() => Ok(Self::NotFound),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, HttpRequest)]
#[http_request(method="GET", response = WellKnownApiBody, path=".well-known/opentalk/api")]
struct WellKnownApiRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct VersionedApiInfo {
    pub base_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ApiInfo {
    pub v1: Option<VersionedApiInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WellKnownApiBody {
    pub opentalk_api: ApiInfo,
}
