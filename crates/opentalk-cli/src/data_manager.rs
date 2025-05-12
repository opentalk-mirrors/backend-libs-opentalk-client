// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fs, path::PathBuf};

use snafu::{OptionExt, ResultExt as _, Snafu};

use crate::{data::AccountData, opntalk_instance_account_id::OpenTalkInstanceAccountId};

#[derive(Debug, Snafu)]
pub enum DataError {
    #[snafu(display("Data can't be loaded from {path:?}"))]
    NotLoadable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Data can't be stored to {path:?}"))]
    NotStorable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Data folder can't be created under {path:?}"))]
    FolderNotCreatable {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("System data home not set"))]
    SystemDataHomeNotSet,

    #[snafu(display("Data not readable from {path:?}"))]
    NotReadable {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[snafu(display("Data not writeable to {path:?}"))]
    NotWriteable {
        path: PathBuf,
        source: toml::ser::Error,
    },
}

/// The DataManager stores and loads configs from providered pathes
#[derive(Debug)]
pub struct DataManager {
    path: PathBuf,
}

impl DataManager {
    /// Returns DataManager instance
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
    pub fn load_instance(&self, id: OpenTalkInstanceAccountId) -> Result<AccountData, DataError> {
        let file = fs::read_to_string(self.data_file_path(id)).context(NotLoadableSnafu {
            path: self.path.clone(),
        })?;

        let data = toml::from_str(file.as_str()).context(NotReadableSnafu {
            path: self.path.clone(),
        })?;

        Ok(data)
    }

    /// Store instaceData
    pub fn store_instance(
        &self,
        id: OpenTalkInstanceAccountId,
        account: AccountData,
    ) -> Result<(), DataError> {
        let data_path = self.data_file_path(id);

        if !data_path.as_path().exists() {
            fs::create_dir_all(&data_path).context(FolderNotCreatableSnafu {
                path: data_path.clone(),
            })?;
        }

        let data_str = toml::to_string_pretty(&account).context(NotWriteableSnafu {
            path: data_path.clone(),
        })?;

        fs::write(&data_path, data_str).context(NotStorableSnafu { path: data_path })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write as _, path::Path};

    use pretty_assertions::{assert_eq, assert_matches};
    use tempfile::tempdir;

    use crate::{
        data::AccountData,
        data_manager::{DataError, DataManager},
    };

    const EXAMPLE_DATA: &str = r#"access_token = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
refresh_token = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
"#;

    fn example() -> AccountData {
        AccountData {
            access_token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
            refresh_token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
        }
    }

    #[test]
    fn success_load_with_empty_data_file() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = fs::create_dir_all(&data_path);

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
        let _ = fs::create_dir_all(&data_path);

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
        let _ = fs::create_dir_all(&data_path);

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
