// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use http_request_derive_client::Client as _;
use http_request_derive_client_reqwest::ReqwestClient;
use opentalk_client_data_persistence::{AccountTokens, DataManager};
use secrecy::SecretString;

use super::{TokenExchangeRequest, TokenExchangeRequestBody};
use crate::{
    Authorization, Client, oidc::OidcEndpoints, oidc_authorization::REFRESH_BEFORE_EXPIRY,
};

/// OIDC Token Exchange grant as per [RFC8693](https://datatracker.ietf.org/doc/html/rfc8693)
#[derive(Debug)]
pub struct OidcTokenExchangeGrant {
    data_manager: Box<dyn DataManager>,
    http_client: ReqwestClient,
    request_body: TokenExchangeRequestBody,
}

#[async_trait::async_trait(?Send)]
impl Authorization for OidcTokenExchangeGrant {
    async fn get_access_token(&self) -> Result<String> {
        self.get_token_and_refresh_if_needed(REFRESH_BEFORE_EXPIRY)
            .await
    }
}

#[async_trait::async_trait(?Send)]
impl Authorization for &OidcTokenExchangeGrant {
    async fn get_access_token(&self) -> Result<String> {
        Authorization::get_access_token(*self).await
    }
}

impl OidcTokenExchangeGrant {
    /// Loads accesss token and fetches a new one, if existing is expired
    async fn get_token_and_refresh_if_needed(
        &self,
        refresh_before_expiry: Duration,
    ) -> Result<String> {
        let AccountTokens {
            access_token_expiry,
            access_token,
            ..
        } = self.data_manager.load_account_tokens()?;

        let now = Utc::now();
        if now + refresh_before_expiry > access_token_expiry {
            Ok(self.get_new_access_token().await?)
        } else {
            Ok(access_token)
        }
    }

    /// Create [`OidcTokenExchangeGrant`] and store access token
    /// Accepts [`Client`] as parameter, as it mostly needs a preconfigured client:
    /// with installed certificates, DNS overrides etc..
    pub async fn create(
        client: &Client,
        data_manager: Box<dyn DataManager>,
        oidc_endpoints: OidcEndpoints,
        oidc_client_id: String,
        oidc_exchange_token: SecretString,
    ) -> Result<Self> {
        let http_client = client
            .reqwest_client()
            .clone()
            .with_base_url(oidc_endpoints.token_endpoint.clone());
        let request_body = TokenExchangeRequestBody {
            client_id: oidc_client_id.clone(),
            subject_token: oidc_exchange_token,
        };
        let grant = Self {
            data_manager,
            http_client,
            request_body,
        };
        let _access_token = grant.get_new_access_token().await?;
        Ok(grant)
    }

    /// Gets a new access token using exchange token
    async fn get_new_access_token(&self) -> Result<String> {
        let body = self.request_body.clone();
        let token_result = self
            .http_client
            .execute(TokenExchangeRequest { body })
            .await?;
        let now = Utc::now();
        let account_tokens = AccountTokens {
            access_token_expiry: now
                + Duration::from_secs(token_result.expires_in.unwrap_or_default()),
            access_token: token_result.access_token,
            refresh_token: token_result
                .refresh_token
                .expect("Refresh token should exist"),
        };

        self.data_manager
            .store_account_tokens(account_tokens.clone())?;
        Ok(account_tokens.access_token)
    }
}
