// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fs, path::PathBuf};

use snafu::{OptionExt, ResultExt as _};

use crate::{
    config::Config,
    config_error::{
        FolderNotCreatableSnafu, NotLoadableSnafu, NotReadableSnafu, NotStorableSnafu,
        NotWriteableSnafu, SystemConfigHomeNotSetSnafu,
    },
    ConfigError,
};

/// The [ConfigManager] stores and loads configs from providered pathes
#[derive(Debug)]
pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    /// Create a new [ConfigManager] instance.
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

    use std::{collections::BTreeMap, io::Write as _, path::Path};

    use pretty_assertions::{assert_eq, assert_matches};
    use tempfile::tempdir;

    use super::Config;
    use crate::{
        config_manager::{ConfigError, ConfigManager},
        opentalk_instance_config::OpenTalkInstanceConfig,
        OpenTalkAccountConfig,
    };

    const EXAMPLE_CONFIG: &str = r#"default_instance = "https://ot.example.com/"

[instances."https://ot.example.com/"]
default_account = "one"

[instances."https://ot.example.com/".accounts.one]
oidc_client_id = "device"

[instances."https://ot.example.com/".accounts.two]
oidc_client_id = "device"

[instances."https://ot01.example.com/"]
default_account = "three"

[instances."https://ot01.example.com/".accounts.three]
oidc_client_id = "device"
"#;

    fn example() -> Config {
        Config {
            default_instance: Some("https://ot.example.com".parse().unwrap()),
            instances: BTreeMap::from_iter([
                (
                    "https://ot.example.com".parse().unwrap(),
                    OpenTalkInstanceConfig {
                        default_account: "one".parse().unwrap(),
                        accounts: BTreeMap::from_iter([
                            (
                                "one".parse().unwrap(),
                                OpenTalkAccountConfig {
                                    oidc_client_id: "device".to_string(),
                                },
                            ),
                            (
                                "two".parse().unwrap(),
                                OpenTalkAccountConfig {
                                    oidc_client_id: "device".to_string(),
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "https://ot01.example.com".parse().unwrap(),
                    OpenTalkInstanceConfig {
                        default_account: "three".parse().unwrap(),
                        accounts: BTreeMap::from_iter([(
                            "three".parse().unwrap(),
                            OpenTalkAccountConfig {
                                oidc_client_id: "device".to_string(),
                            },
                        )]),
                    },
                ),
            ]),
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

        assert_eq!(Config::default(), conf);
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

        assert_eq!(example(), conf);
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

        assert_eq!(EXAMPLE_CONFIG, stored_config);
    }

    #[test]
    fn success_new() {
        std::env::set_var("XDG_CONFIG_HOME", "/home/example/.config");
        let config_manager = ConfigManager::new().unwrap();
        assert_eq!(
            Path::new("/home/example/.config/opentalk/cli.toml"),
            config_manager.path,
        );
    }
}
