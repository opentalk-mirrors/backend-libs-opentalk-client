// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fs, path::PathBuf};

use snafu::{OptionExt, ResultExt as _, Snafu};

use crate::config::Config;

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Config can't be loaded from {path:?}"))]
    NotLoadable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Config can't be stored to {path:?}"))]
    NotStorable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Config folder can't be created under {path:?}"))]
    FolderNotCreatable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("System config home not set"))]
    SystemConfigHomeNotSet,

    #[snafu(display("Config not readable from {path:?}"))]
    NotReadable {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[snafu(display("Config not writeable to {path:?}"))]
    NotWriteable {
        path: PathBuf,
        source: toml::ser::Error,
    },
}

/// The ConfigManager stores and loads configs from providered pathes
pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = dirs::config_dir()
            .context(SystemConfigHomeNotSetSnafu)?
            .join("opentalk/cli.toml");

        Ok(Self { path: config_path })
    }
    /// Load config if exist
    pub fn load(&self) -> Result<Config, ConfigError> {
        let file = fs::read_to_string(&self.path).context(NotLoadableSnafu {
            path: self.path.clone(),
        })?;
        let config = toml::from_str(file.as_str()).context(NotReadableSnafu {
            path: self.path.clone(),
        })?;

        Ok(config)
    }

    /// Store config
    pub fn store(&self, config: &Config) -> Result<(), ConfigError> {
        if let Some(config_dir) = self.path.parent() {
            fs::create_dir_all(config_dir).context(FolderNotCreatableSnafu { path: config_dir })?;
        }

        let config_str = toml::to_string_pretty(config).context(NotWriteableSnafu {
            path: self.path.clone(),
        })?;

        fs::write(&self.path, config_str).context(NotStorableSnafu {
            path: self.path.clone(),
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{io::Write as _, path::Path};

    use pretty_assertions::{assert_eq, assert_matches};
    use tempfile::tempdir;
    use url::Url;

    use super::Config;
    use crate::{
        config::{OpenTalkAccount, OpenTalkInstance},
        config_manager::{ConfigError, ConfigManager},
    };

    const EXAMPLE_CONFIG: &str = r"default_instance_url = 'https://ot.example.com/'

[[instances]]
url = 'https://ot.example.com/'
default_account_name = 'one'

[[instances.accounts]]
oidc_client_id = 'device'
name = 'one'

[[instances.accounts]]
oidc_client_id = 'device'
name = 'two'

[[instances]]
url = 'https://ot01.example.com/'
default_account_name = 'three'

[[instances.accounts]]
oidc_client_id = 'device'
name = 'three'
";

    fn example() -> Config {
        Config {
            default_instance_url: Some(Url::parse("https://ot.example.com").unwrap()),
            instances: vec![
                OpenTalkInstance {
                    url: Url::parse("https://ot.example.com").unwrap(),
                    default_account_name: "one".to_string(),
                    accounts: vec![
                        OpenTalkAccount {
                            oidc_client_id: "device".to_string(),
                            name: "one".to_string(),
                        },
                        OpenTalkAccount {
                            oidc_client_id: "device".to_string(),
                            name: "two".to_string(),
                        },
                    ],
                },
                OpenTalkInstance {
                    url: Url::parse("https://ot01.example.com").unwrap(),
                    default_account_name: "three".to_string(),
                    accounts: vec![OpenTalkAccount {
                        oidc_client_id: "device".to_string(),
                        name: "three".to_string(),
                    }],
                },
            ],
        }
    }

    #[test]
    fn success_load_with_empty_config() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let config_path = temp_dir.path().join("opentalk-cli.toml");
        {
            let _ = std::fs::File::create(&config_path).unwrap();
        }
        let config_manager = ConfigManager {
            path: config_path.clone(),
        };
        let conf = config_manager.load().unwrap();

        assert_eq!(conf, Config::default());
    }

    #[test]
    fn error_load_with_missing_file() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let config_path = temp_dir.path().join("opentalk-cli.toml");
        let config_manager = ConfigManager {
            path: config_path.clone(),
        };
        let conf = config_manager.load();

        assert_matches!(conf, Err(ConfigError::NotLoadable { path, source: _ }) if path == config_path);
    }

    #[test]
    fn success_load_example_config() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let config_path = temp_dir.path().join("opentalk-cli.toml");
        {
            let mut file = std::fs::File::create(&config_path).unwrap();
            write!(file, "{EXAMPLE_CONFIG}").unwrap()
        }
        let config_manager = ConfigManager {
            path: config_path.clone(),
        };
        let conf = config_manager.load().unwrap();

        assert_eq!(conf, example());
    }

    #[test]
    fn success_store_config() {
        let config = example();

        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config/opentalk-cli.toml");

        let config_manager = ConfigManager {
            path: config_path.clone(),
        };

        config_manager.store(&config).unwrap();

        let stored_config = std::fs::read_to_string(&config_path).unwrap();

        assert_eq!(stored_config, EXAMPLE_CONFIG);
    }

    #[test]
    fn success_new() {
        std::env::set_var("XDG_CONFIG_HOME", "/home/example/.config");
        let config_manager = ConfigManager::new().unwrap();
        assert_eq!(
            config_manager.path,
            Path::new("/home/example/.config/opentalk/cli.toml")
        );
    }
}
