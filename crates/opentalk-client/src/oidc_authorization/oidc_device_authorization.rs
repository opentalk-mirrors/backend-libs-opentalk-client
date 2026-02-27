// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use oauth2::{
    AuthUrl, ClientId, DeviceAuthorizationUrl, RefreshToken, Scope,
    StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl, basic::BasicClient,
};
use opentalk_client_data_persistence::{AccountTokens, DataManager, OpenTalkInstanceAccountId};

use crate::{Authorization, oidc::OidcEndpoints, oidc_authorization::REFRESH_BEFORE_EXPIRY};

/// TODO
#[derive(Debug)]
pub struct OidcDeviceAuthorization {
    instance_account_id: OpenTalkInstanceAccountId,
    data_manager: Box<dyn DataManager>,
    oidc_endpoints: OidcEndpoints,
    oidc_client_id: String,
}

#[async_trait::async_trait(?Send)]
impl Authorization for OidcDeviceAuthorization {
    async fn get_access_token(&self) -> Result<String> {
        self.get_token_and_refresh_if_needed(REFRESH_BEFORE_EXPIRY)
            .await
    }
}

#[async_trait::async_trait(?Send)]
impl Authorization for &OidcDeviceAuthorization {
    async fn get_access_token(&self) -> Result<String> {
        Authorization::get_access_token(*self).await
    }
}

impl OidcDeviceAuthorization {
    /// TODO
    pub async fn load_from_datamanager(
        data_manager: Box<dyn DataManager>,
        oidc_client_id: String,
        instance_account_id: &OpenTalkInstanceAccountId,
        oidc_endpoints: OidcEndpoints,
    ) -> Result<Self> {
        let _ = data_manager.load_instance(instance_account_id)?;

        Ok(Self {
            instance_account_id: instance_account_id.clone(),
            data_manager,
            oidc_endpoints,
            oidc_client_id,
        })
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

        let builder = reqwest::ClientBuilder::new();

        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        let builder = {
            // Following redirects opens the client up to SSRF vulnerabilities.
            builder.redirect(reqwest::redirect::Policy::none())
        };

        let http_client = builder.build().expect("Client should build");
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

    /// Loads access token
    pub async fn get_token(self) -> Result<String> {
        Ok(self
            .data_manager
            .load_instance(&self.instance_account_id)?
            .access_token)
    }

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

    /// perform oidc device authorization
    pub async fn create_with_device_authorization(
        data_manager: Box<dyn DataManager>,
        oidc_endpoints: OidcEndpoints,
        oidc_client_id: String,
        instance_account_id: &OpenTalkInstanceAccountId,
    ) -> Result<Self> {
        let device_auth_url =
            DeviceAuthorizationUrl::new(oidc_endpoints.device_authorization_endpoint.to_string())
                .unwrap();

        let oidc_client = BasicClient::new(ClientId::new(oidc_client_id.clone()))
            .set_auth_uri(AuthUrl::new(oidc_endpoints.authorization_endpoint.to_string()).unwrap())
            .set_token_uri(TokenUrl::new(oidc_endpoints.token_endpoint.to_string()).unwrap())
            .set_device_authorization_url(device_auth_url);

        let builder = reqwest::ClientBuilder::new();

        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        let builder = {
            // Following redirects opens the client up to SSRF vulnerabilities.
            builder.redirect(reqwest::redirect::Policy::none())
        };

        let http_client = builder.build().expect("Client should build");
        let http_client = super::ClientWrapper(http_client);

        let pull_client = oidc_client.clone();

        let details: StandardDeviceAuthorizationResponse = pull_client
            .exchange_device_code()
            .add_scope(Scope::new("profile email openid".to_string()))
            .request_async(&http_client)
            .await
            .unwrap();

        println!(
            "Open this URL in your browser:\n{}?user_code={}\nand enter the code: {}",
            details.verification_uri(),
            details.user_code().secret(),
            details.user_code().secret()
        );

        let now = Utc::now();

        let token_result = oidc_client
            .exchange_device_access_token(&details)
            .request_async(&http_client, tokio::time::sleep, None)
            .await
            .unwrap();

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
