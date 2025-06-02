// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use anyhow::Result;
use clap::Parser;
use opentalk_client::Client;
use url::Url;

mod config;
mod config_manager;
mod opentalk_account_id;
mod opentalk_instance_id;

#[derive(Debug, Parser)]
enum Command {
    /// Discover the API based on an OpenTalk frontend URL
    ///
    /// This is just an example command that allows to test out the basic interaction with the OpenTalk API.
    Discover { client_url: Url },
}

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::parse();

    match command {
        Command::Discover { client_url } => {
            discover(client_url).await?;
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
