// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use http::{HeaderValue, Method, Request, Response, header::AUTHORIZATION};
use http_request_derive::{Error, HttpRequest};
use url::Url;

use crate::Authorization;

/// A client authenticated to the OIDC provider.
#[derive(Debug)]
pub struct AuthenticatedClient<C, A> {
    inner: C,
    authorization: A,
}

impl<C, A> AuthenticatedClient<C, A> {
    /// Create a new [AuthenticatedClient] with a valid authorization.
    pub fn new(inner: C, authorization: A) -> Self {
        Self {
            inner,
            authorization,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl<C: http_request_derive_client::Client, A: Authorization + Sync>
    http_request_derive_client::Client for AuthenticatedClient<C, A>
{
    /// An error that can be returned during request execution by the [`AuthenticatedClient`].
    type ClientError = C::ClientError;

    /// Execute a [`http_request_derive::HttpRequest`], and read the typed response.
    async fn execute<R: HttpRequest + Send>(
        &self,
        request: R,
    ) -> Result<R::Response, Self::ClientError> {
        let bearer_token = self.authorization.get_access_token().await.unwrap();

        let request = AuthenticatedRequest {
            request,
            bearer_token,
        };
        self.inner.execute(request).await
    }
}

struct AuthenticatedRequest<R> {
    request: R,
    bearer_token: String,
}

impl<R: HttpRequest> HttpRequest for AuthenticatedRequest<R> {
    type Response = R::Response;
    type Query = R::Query;
    type Body = R::Body;

    const METHOD: Method = R::METHOD;

    fn path(&self) -> String {
        self.request.path()
    }

    fn query(&self) -> Option<&Self::Query> {
        self.request.query()
    }

    fn body(&self) -> Option<&Self::Body> {
        self.request.body()
    }

    fn to_http_request(&self, base_url: &Url) -> Result<Request<Vec<u8>>, Error> {
        let mut request = self.request.to_http_request(base_url)?;
        _ = request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.bearer_token)).unwrap(),
        );
        Ok(request)
    }

    fn read_response(response: Response<Bytes>) -> Result<Self::Response, Error> {
        R::read_response(response)
    }
}
