// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2
use anyhow::{Result, anyhow};
use opentalk_client_data_persistence::DataManager;

use super::{OidcDeviceAuthorization, OidcDirectAccessGrant, OidcTokenExchangeGrant};
use crate::{Authorization, Client, OidcAuthMethod, OidcEndpoints};

/// A convenience facade, which will decide which kind of supported grant to use for user
/// authentication depending on [`crate::OidcAuthMethod`] provided
#[derive(Debug)]
pub enum OidcGrant {
    /// Direct Access ([Resource Owner Password Credentials](https://datatracker.ietf.org/doc/html/rfc6749#section-4.3))
    DirectAccess(OidcDirectAccessGrant),
    /// [Token Exchange grant](https://datatracker.ietf.org/doc/html/rfc8693)   
    TokenExchange(OidcTokenExchangeGrant),
    /// [DeviceAuthorization](https://datatracker.ietf.org/doc/html/rfc8628)   
    DeviceAuthorization(OidcDeviceAuthorization),
}

#[async_trait::async_trait(?Send)]
impl Authorization for OidcGrant {
    async fn get_access_token(&self) -> Result<String> {
        match self {
            OidcGrant::DirectAccess(grant) => grant.get_access_token().await,
            OidcGrant::TokenExchange(grant) => grant.get_access_token().await,
            OidcGrant::DeviceAuthorization(grant) => grant.get_access_token().await,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl Authorization for &OidcGrant {
    async fn get_access_token(&self) -> Result<String> {
        Authorization::get_access_token(*self).await
    }
}

impl OidcGrant {
    /// Authenticate user based on provided credentials
    pub async fn authenticate(
        client: Option<&Client>,
        data_manager: Box<dyn DataManager>,
        oidc_endpoints: OidcEndpoints,
        oidc_client_id: String,
        method: OidcAuthMethod,
    ) -> Result<Self> {
        match method {
            OidcAuthMethod::DirectAccess { username, password } => {
                let grant = OidcDirectAccessGrant::create_with_direct_access_grant(
                    data_manager,
                    oidc_endpoints,
                    oidc_client_id,
                    username,
                    password,
                )
                .await?;
                Ok(Self::DirectAccess(grant))
            }
            OidcAuthMethod::TokenExchange { subject_token } => {
                let client = client.ok_or_else(|| {
                    anyhow!("OIDC Exchange Token grant needs a preconfigured http client")
                })?;
                let grant = OidcTokenExchangeGrant::create(
                    client,
                    data_manager,
                    oidc_endpoints,
                    oidc_client_id,
                    subject_token,
                )
                .await?;
                Ok(Self::TokenExchange(grant))
            }
            OidcAuthMethod::DeviceAuthorization => {
                let grant = OidcDeviceAuthorization::create_with_device_authorization(
                    data_manager,
                    oidc_endpoints,
                    oidc_client_id,
                )
                .await?;
                Ok(Self::DeviceAuthorization(grant))
            }
        }
    }
}
