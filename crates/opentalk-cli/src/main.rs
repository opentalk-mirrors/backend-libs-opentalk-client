// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use anyhow::{Result, bail};
use clap::Parser;
use opentalk_client::{Client, OidcDeviceAuthorization};
use opentalk_client_data_persistence::{
    ConfigManager, DataManager, OpenTalkAccountConfig, OpenTalkAccountId, OpenTalkInstanceConfig,
    OpenTalkInstanceId,
};
use opentalk_client_requests_api_v1::EventsGetRequest;
use opentalk_types_api_v1::events::{EventExceptionResource, EventOrException, EventResource};
use url::Url;

#[derive(Debug, Parser)]
enum Command {
    /// Discover the API based on an OpenTalk frontend URL
    ///
    /// This is just an example command that allows to test out the basic interaction with the OpenTalk API.
    Discover {
        client_url: Url,
    },

    /// Login to a new OpenTalk instance
    Login {
        instance_url: OpenTalkInstanceId,
        account_id: OpenTalkAccountId,
        oidc_client_id: String,
    },

    /// Logout
    Logout {
        #[arg(long)]
        instance_url: Option<OpenTalkInstanceId>,

        #[arg(long)]
        account_id: Option<OpenTalkAccountId>,
    },
    // List Accounts
    ListAccounts,

    // List Events
    ListEvents {
        #[arg(long)]
        instance_url: Option<OpenTalkInstanceId>,

        #[arg(long)]
        account_id: Option<OpenTalkAccountId>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::parse();

    match command {
        Command::Discover { client_url } => {
            discover(client_url).await?;
        }
        Command::Login {
            instance_url,
            account_id,
            oidc_client_id,
        } => {
            login(instance_url, account_id, oidc_client_id).await?;
        }
        Command::Logout {
            instance_url,
            account_id,
        } => {
            logout(instance_url, account_id)?;
        }
        Command::ListAccounts => {
            list_accounts()?;
        }
        Command::ListEvents {
            instance_url,
            account_id,
        } => {
            list_events(instance_url, account_id).await?;
        }
    }

    Ok(())
}

async fn discover(client_url: Url) -> Result<()> {
    let client = Client::discover(client_url).await?;
    let oidc_provider = client.get_oidc_provider().await?;
    println!("Discovered OIDC provider:\n{oidc_provider:#?}");
    Ok(())
}

fn list_accounts() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load().unwrap_or_default();

    config.instances.iter().for_each(|instance| {
        println!("Instance: {}", **instance.0);
        println!(" {:<20} {:<20}", "Accounts:", "OIDC Device ID:");
        instance.1.accounts.iter().for_each(|account| {
            println!(" {:<20} {:<20}", **account.0, account.1.oidc_client_id,);
        });
        println!();
    });

    Ok(())
}

async fn list_events(
    instance_id: Option<OpenTalkInstanceId>,
    account_id: Option<OpenTalkAccountId>,
) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load().unwrap();
    let Some((instance_account_id, account)) =
        config.get_account_with_fallback_to_default(instance_id.as_ref(), account_id.as_ref())
    else {
        bail!("no account found");
    };

    let instance_id = instance_account_id.instance_id().clone();

    let client = Client::discover(instance_id.into()).await?;
    let oidc_endpoints = client.get_oidc_endpoints().await?;

    let data_manager = DataManager::new()?;

    let authorization = OidcDeviceAuthorization::load_from_datamanager(
        data_manager,
        account.oidc_client_id,
        &instance_account_id,
        oidc_endpoints,
    )
    .await?;

    let response = client
        .execute_authorized(EventsGetRequest, authorization)
        .await?;

    for item in response {
        match item {
            EventOrException::Event(EventResource {
                id,
                title,
                description,
                ..
            }) => {
                println!("Event {id}");
                println!("  Title: {title}");
                println!("  Description: {description}");
            }
            EventOrException::Exception(EventExceptionResource {
                recurring_event_id,
                instance_id,
                ..
            }) => {
                println!("Exception {instance_id} for event {recurring_event_id}");
            }
        }
    }

    Ok(())
}

fn logout(
    instance_id: Option<OpenTalkInstanceId>,
    account_id: Option<OpenTalkAccountId>,
) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut config = config_manager.load().unwrap_or_default();

    config.remove_account(instance_id.as_ref(), account_id.as_ref());

    config_manager.store(&config)?;

    Ok(())
}

async fn login(
    instance_id: OpenTalkInstanceId,
    account_id: OpenTalkAccountId,
    oidc_client_id: String,
) -> Result<()> {
    let data_manager = DataManager::new()?;

    let client = Client::discover(instance_id.clone().into()).await?;
    let oidc_endpoints = client.get_oidc_endpoints().await?;

    let _authorization = OidcDeviceAuthorization::create_with_device_authorization(
        data_manager,
        oidc_endpoints,
        oidc_client_id.clone(),
        &(instance_id.clone(), account_id.clone()).into(),
    )
    .await?;

    let config_manager = ConfigManager::new()?;
    let mut config = config_manager.load().unwrap_or_default();

    let instance = match config.instances.get(&instance_id) {
        Some(current_instance) => {
            let mut tmp_instance = current_instance.clone();
            tmp_instance.accounts.insert(
                account_id.clone(),
                OpenTalkAccountConfig {
                    oidc_client_id: oidc_client_id.clone(),
                },
            );
            tmp_instance
        }
        None => OpenTalkInstanceConfig {
            default_account: account_id.clone(),
            accounts: BTreeMap::from_iter([(
                account_id.clone(),
                OpenTalkAccountConfig {
                    oidc_client_id: oidc_client_id.clone(),
                },
            )]),
        },
    };

    config
        .instances
        .insert(instance_id.clone(), instance.clone());
    if config.default_instance.is_none() {
        config.default_instance = Some(instance_id);
    }

    config_manager.store(&config)?;

    Ok(())
}
