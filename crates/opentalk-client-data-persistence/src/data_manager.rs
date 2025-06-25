// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::path::PathBuf;

use snafu::{OptionExt as _, ResultExt as _};

use crate::{
    account_data_file::AccountDataFile,
    data_error::{
        FolderNotCreatableSnafu, NotLoadableSnafu, NotReadableSnafu, NotStorableSnafu,
        NotWriteableSnafu, SystemDataHomeNotSetSnafu,
    },
    AccountTokens, DataError, OpenTalkInstanceAccountId,
};

/// The [DataManager] stores and loads configs from providered pathes
#[derive(Debug)]
pub struct DataManager {
    path: PathBuf,
}

impl DataManager {
    /// Returns [DataManager] instance
    pub fn new() -> Result<Self, DataError> {
        let data_path = dirs::data_dir()
            .context(SystemDataHomeNotSetSnafu)?
            .join("opentalk/cli");

        Ok(Self { path: data_path })
    }

    fn data_file_path(&self, id: OpenTalkInstanceAccountId) -> PathBuf {
        self.path.join(format!("{}.toml", id.to_file_stem()))
    }

    /// Load instaceData
    pub fn load_instance(&self, id: OpenTalkInstanceAccountId) -> Result<AccountTokens, DataError> {
        let file = std::fs::read_to_string(self.data_file_path(id)).context(NotLoadableSnafu {
            path: self.path.clone(),
        })?;

        let data: AccountDataFile = toml::from_str(file.as_str()).context(NotReadableSnafu {
            path: self.path.clone(),
        })?;

        Ok(data.opentalk_account_tokens)
    }

    /// Store instaceData
    pub fn store_instance(
        &self,
        id: OpenTalkInstanceAccountId,
        opentalk_account_tokens: AccountTokens,
    ) -> Result<(), DataError> {
        let data_path = self.data_file_path(id);

        if !data_path.as_path().exists() {
            std::fs::create_dir_all(&data_path).context(FolderNotCreatableSnafu {
                path: data_path.clone(),
            })?;
        }

        let data_str = toml::to_string_pretty(&AccountDataFile {
            opentalk_account_tokens,
        })
        .context(NotWriteableSnafu {
            path: data_path.clone(),
        })?;

        std::fs::write(&data_path, data_str).context(NotStorableSnafu { path: data_path })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write as _, path::Path};

    use pretty_assertions::{assert_eq, assert_matches};
    use tempfile::tempdir;

    use super::DataManager;
    use crate::{AccountTokens, DataError};

    const EXAMPLE_DATA: &str = r#"[opentalk_account_tokens]
access_token_expiry = "2025-01-01T02:03:04Z"
access_token = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
refresh_token = "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
"#;

    fn example() -> AccountTokens {
        AccountTokens {
            access_token_expiry: "2025-01-01T02:03:04Z".parse().unwrap(),
            access_token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
            refresh_token: "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy".to_string(),
        }
    }

    #[test]
    fn success_load_with_empty_data_file() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = std::fs::create_dir_all(&data_path);

        let data_manager = DataManager {
            path: data_path.clone(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let _ = std::fs::File::create(&account_data_file_path).unwrap();
        }

        let data = data_manager.load_instance(
            (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        );

        assert_matches!(data, Err(DataError::NotReadable { path, source: _ }) if path == data_path);
    }

    #[test]
    fn error_load_with_missing_file() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let data_manager = DataManager {
            path: data_path.clone(),
        };

        let data = data_manager.load_instance(
            (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        );

        assert_matches!(data, Err(DataError::NotLoadable { path, source: _ }) if path == data_path);
    }

    #[test]
    fn success_load_with_example_data() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = std::fs::create_dir_all(&data_path);

        let data_manager = DataManager {
            path: data_path.clone(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let mut file = std::fs::File::create(&account_data_file_path).unwrap();
            write!(file, "{EXAMPLE_DATA}").unwrap();
        }

        let data = data_manager
            .load_instance(
                (
                    "http://example_instance.example".parse().unwrap(),
                    "example-account".parse().unwrap(),
                )
                    .into(),
            )
            .unwrap();

        assert_eq!(data, example());
    }

    #[test]
    fn success_store_data() {
        let data = example();

        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = std::fs::create_dir_all(&data_path);

        let data_manager = DataManager {
            path: data_path.clone(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let mut file = std::fs::File::create(&account_data_file_path).unwrap();
            write!(file, "{EXAMPLE_DATA}").unwrap();
        }

        data_manager
            .store_instance(
                (
                    "http://example_instance.example".parse().unwrap(),
                    "example-account".parse().unwrap(),
                )
                    .into(),
                data,
            )
            .unwrap();

        let stored_data = std::fs::read_to_string(&account_data_file_path).unwrap();

        assert_eq!(stored_data, EXAMPLE_DATA);
    }

    #[test]
    fn success_new() {
        std::env::set_var("XDG_DATA_HOME", "/tmp/test/.local/share/");
        let config_manager = DataManager::new().unwrap();
        assert_eq!(
            config_manager.path,
            Path::new("/tmp/test/.local/share/opentalk/cli")
        );
    }
}
