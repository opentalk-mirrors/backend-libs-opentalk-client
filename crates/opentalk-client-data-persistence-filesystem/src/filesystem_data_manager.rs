// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::{fs, path::PathBuf};

use opentalk_client_data_persistence::{
    AccountTokens, DataError, DataManager, OpenTalkInstanceAccountId,
};
use snafu::{OptionExt as _, ResultExt as _};

use crate::{
    FilesystemDataError,
    account_data_file::AccountDataFile,
    filesystem_data_error::{
        FolderNotCreatableSnafu, NotLoadableSnafu, NotReadableSnafu, NotStorableSnafu,
        NotWriteableSnafu, SystemDataHomeNotSetSnafu,
    },
};

/// The [FilesystemDataManager] stores and loads local data from provided pathes
#[derive(Debug)]
pub struct FilesystemDataManager {
    path: PathBuf,
    instance_account_id: OpenTalkInstanceAccountId,
}

impl DataManager for FilesystemDataManager {
    fn load_account_tokens(&self) -> Result<AccountTokens, DataError> {
        Ok(FilesystemDataManager::load_account_tokens(self)?)
    }

    fn store_account_tokens(
        &self,
        opentalk_account_tokens: AccountTokens,
    ) -> Result<(), DataError> {
        Ok(FilesystemDataManager::store_instance(
            self,
            opentalk_account_tokens,
        )?)
    }
}

impl FilesystemDataManager {
    /// Returns [FilesystemDataManager] instance
    pub fn new(
        instance_account_id: OpenTalkInstanceAccountId,
    ) -> Result<Self, FilesystemDataError> {
        let data_path = dirs::data_dir()
            .context(SystemDataHomeNotSetSnafu)?
            .join("opentalk/cli");

        Ok(Self {
            path: data_path,
            instance_account_id,
        })
    }

    fn data_file_path(&self) -> PathBuf {
        self.path
            .join(format!("{}.toml", self.instance_account_id.to_file_stem()))
    }

    fn load_account_tokens(&self) -> Result<AccountTokens, FilesystemDataError> {
        let file = fs::read_to_string(self.data_file_path()).context(NotLoadableSnafu {
            path: self.path.clone(),
        })?;

        let data: AccountDataFile = toml::from_str(file.as_str()).context(NotReadableSnafu {
            path: self.path.clone(),
        })?;

        Ok(data.opentalk_account_tokens)
    }

    fn store_instance(
        &self,
        opentalk_account_tokens: AccountTokens,
    ) -> Result<(), FilesystemDataError> {
        let data_path = self.data_file_path();

        if let Some(data_dir) = data_path.parent() {
            fs::create_dir_all(data_dir).context(FolderNotCreatableSnafu { path: data_dir })?;
        }

        let data_str = toml::to_string_pretty(&AccountDataFile {
            opentalk_account_tokens,
        })
        .context(NotWriteableSnafu {
            path: data_path.clone(),
        })?;

        fs::write(&data_path, data_str).context(NotStorableSnafu { path: data_path })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write as _, path::Path};

    use opentalk_client_data_persistence::AccountTokens;
    use pretty_assertions::{assert_eq, assert_matches};
    use tempfile::tempdir;

    use super::FilesystemDataManager;
    use crate::FilesystemDataError;

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

        let data_manager = FilesystemDataManager {
            path: data_path.clone(),
            instance_account_id: (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let _ = std::fs::File::create(&account_data_file_path).unwrap();
        }

        let data = data_manager.load_account_tokens();

        assert_matches!(data, Err(FilesystemDataError::NotReadable { path, source: _ }) if path == data_path);
    }

    #[test]
    fn error_load_with_missing_file() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let data_manager = FilesystemDataManager {
            path: data_path.clone(),
            instance_account_id: (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        };

        let data = data_manager.load_account_tokens();

        assert_matches!(data, Err(FilesystemDataError::NotLoadable { path, source: _ }) if path == data_path);
    }

    #[test]
    fn success_load_with_example_data() {
        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = std::fs::create_dir_all(&data_path);

        let data_manager = FilesystemDataManager {
            path: data_path.clone(),
            instance_account_id: (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let mut file = std::fs::File::create(&account_data_file_path).unwrap();
            write!(file, "{EXAMPLE_DATA}").unwrap();
        }

        let data = data_manager.load_account_tokens().unwrap();

        assert_eq!(data, example());
    }

    #[test]
    fn success_store_data() {
        let data = example();

        // Create a directory inside of `env::temp_dir()`.
        let temp_dir = tempdir().unwrap();

        let data_path = temp_dir.path().join("data");
        let _ = std::fs::create_dir_all(&data_path);

        let data_manager = FilesystemDataManager {
            path: data_path.clone(),
            instance_account_id: (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        };

        let account_data_file_path =
            data_path.join("example_instance.example___example-account.toml");
        {
            let mut file = std::fs::File::create(&account_data_file_path).unwrap();
            write!(file, "{EXAMPLE_DATA}").unwrap();
        }

        data_manager.store_instance(data).unwrap();

        let stored_data = std::fs::read_to_string(&account_data_file_path).unwrap();

        assert_eq!(stored_data, EXAMPLE_DATA);
    }

    #[test]
    fn success_new() {
        #[allow(unsafe_code)]
        unsafe {
            // We only run this inside a single test, we just need to make sure
            // that we don't set `XDG_DATA_HOME` anywhere else.
            std::env::set_var("XDG_DATA_HOME", "/tmp/test/.local/share/");
        }
        let config_manager = FilesystemDataManager::new(
            (
                "http://example_instance.example".parse().unwrap(),
                "example-account".parse().unwrap(),
            )
                .into(),
        )
        .unwrap();
        assert_eq!(
            config_manager.path,
            Path::new("/tmp/test/.local/share/opentalk/cli")
        );
    }
}
