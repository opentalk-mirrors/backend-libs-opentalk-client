// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use oauth2::{
    AuthUrl, ClientId, RefreshToken, ResourceOwnerPassword, ResourceOwnerUsername, Scope,
    TokenResponse as _, TokenUrl, basic::BasicClient,
};
use opentalk_client_data_persistence::{AccountTokens, DataManager, OpenTalkInstanceAccountId};
use secrecy::{ExposeSecret, SecretString};

use crate::{Authorization, oidc::OidcEndpoints, oidc_authorization::REFRESH_BEFORE_EXPIRY};

/// TODO
#[derive(Debug)]
pub struct OidcDirectAccessGrant {
    instance_account_id: OpenTalkInstanceAccountId,
    data_manager: DataManager,
    oidc_endpoints: OidcEndpoints,
    oidc_client_id: String,
}

#[async_trait::async_trait]
impl Authorization for OidcDirectAccessGrant {
    async fn get_access_token(&self) -> Result<String> {
        self.get_token_and_refresh_if_needed(REFRESH_BEFORE_EXPIRY)
            .await
    }
}

#[async_trait::async_trait]
impl Authorization for &OidcDirectAccessGrant {
    async fn get_access_token(&self) -> Result<String> {
        Authorization::get_access_token(*self).await
    }
}

impl OidcDirectAccessGrant {
    /// Loads accesss token and calls refresh if needed
    pub async fn get_token_and_refresh_if_needed(
        &self,
        refresh_before_expiry: Duration,
    ) -> Result<String> {
        let AccountTokens {
            access_token_expiry,
            access_token,
            ..
        } = self.data_manager.load_instance(&self.instance_account_id)?;

        let now = Utc::now();
        if now + refresh_before_expiry > access_token_expiry {
            Ok(self.refresh_token().await?)
        } else {
            Ok(access_token)
        }
    }

    /// Performs token refresh
    pub async fn refresh_token(&self) -> Result<String> {
        let AccountTokens { refresh_token, .. } =
            self.data_manager.load_instance(&self.instance_account_id)?;

        let client = BasicClient::new(ClientId::new(self.oidc_client_id.clone()))
            .set_auth_uri(
                AuthUrl::new(self.oidc_endpoints.authorization_endpoint.to_string()).unwrap(),
            )
            .set_token_uri(TokenUrl::new(self.oidc_endpoints.token_endpoint.to_string()).unwrap());

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");
        let http_client = super::ClientWrapper(http_client);

        let response = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(&http_client)
            .await
            .unwrap();

        let now = Utc::now();
        let account_tokens = AccountTokens {
            access_token_expiry: now + response.expires_in().unwrap_or_default(),
            access_token: response.access_token().secret().clone(),
            refresh_token: response.refresh_token().unwrap().secret().clone(),
        };
        let _ = self
            .data_manager
            .store_instance(&self.instance_account_id, account_tokens.clone());

        Ok(account_tokens.access_token)
    }

    /// perform oidc direct access grand authorization
    pub async fn create_with_direct_access_grant(
        data_manager: DataManager,
        oidc_endpoints: OidcEndpoints,
        oidc_client_id: String,
        oidc_user: String,
        oidc_password: SecretString,
        instance_account_id: &OpenTalkInstanceAccountId,
    ) -> Result<Self> {
        let oidc_client = BasicClient::new(ClientId::new(oidc_client_id.clone()))
            .set_auth_uri(AuthUrl::new(oidc_endpoints.authorization_endpoint.to_string()).unwrap())
            .set_token_uri(TokenUrl::new(oidc_endpoints.token_endpoint.to_string()).unwrap());

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");
        let http_client = super::ClientWrapper(http_client);

        let token_result = oidc_client
            .exchange_password(
                &ResourceOwnerUsername::new(oidc_user.clone()),
                &ResourceOwnerPassword::new(oidc_password.expose_secret().to_string()),
            )
            .add_scope(Scope::new("openid".to_string()))
            .request_async(&http_client)
            .await
            .unwrap();

        let now = Utc::now();

        let account_tokens = AccountTokens {
            access_token_expiry: now + token_result.expires_in().unwrap_or_default(),
            access_token: token_result.access_token().clone().into_secret(),
            refresh_token: token_result
                .refresh_token()
                .expect("Refresh token should be exist")
                .clone()
                .into_secret(),
        };

        data_manager.store_instance(instance_account_id, account_tokens.clone())?;

        println!("{:?}", token_result);

        Ok(Self {
            instance_account_id: instance_account_id.clone(),
            data_manager,
            oidc_endpoints,
            oidc_client_id,
        })
    }
}
