// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::BTreeMap;

use anyhow::Result;
use clap::Parser;
use opentalk_client::Client;
use opentalk_client_data_persistence::{
    ConfigManager, OpenTalkAccountConfig, OpenTalkAccountId, OpenTalkInstanceConfig,
    OpenTalkInstanceId,
};
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
        instance_url: OpenTalkInstanceId,
        account_id: OpenTalkAccountId,
    },
    ListAccounts,
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
    let conf_manager = ConfigManager::new()?;
    let conf = conf_manager.load().unwrap_or_default();

    conf.instances.iter().for_each(|instance| {
        println!("Instance: {}", **instance.0);
        println!(" {:<20} {:<20}", "Accounts:", "OIDC Device ID:");
        instance.1.accounts.iter().for_each(|account| {
            println!(" {:<20} {:<20}", **account.0, account.1.oidc_client_id,);
        });
        println!();
    });

    Ok(())
}

fn logout(instance_url: OpenTalkInstanceId, account_id: OpenTalkAccountId) -> Result<()> {
    let conf_manager = ConfigManager::new()?;
    let mut conf = conf_manager.load().unwrap_or_default();

    let instance = match conf.instances.get(&instance_url) {
        Some(current_instance) => {
            let mut tmp_instance = current_instance.clone();
            tmp_instance.accounts.remove(&account_id.clone());
            Some(tmp_instance)
        }
        None => None,
    };

    if let Some(instance_value) = instance {
        if !instance_value.accounts.is_empty() {
            conf.instances.insert(instance_url, instance_value);
        } else {
            conf.instances.remove(&instance_url);
        }
    }

    conf_manager.store(&conf)?;

    Ok(())
}

async fn login(
    instance_url: OpenTalkInstanceId,
    account_id: OpenTalkAccountId,
    oidc_client_id: String,
) -> Result<()> {
    let _client = Client::discover(instance_url.as_ref().clone()).await?;
    // Perform oidc device auth

    let conf_manager = ConfigManager::new()?;
    let mut conf = conf_manager.load().unwrap_or_default();

    let instance = match conf.instances.get(&instance_url) {
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

    conf.instances.insert(instance_url, instance.clone());
    conf_manager.store(&conf)?;

    Ok(())
}
