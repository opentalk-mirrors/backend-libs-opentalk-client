// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::path::PathBuf;

use snafu::Snafu;

/// The error returned from config functions in this crate.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ConfigError {
    /// Config can't be loaded from a path.
    #[snafu(display("Config can't be loaded from {path:?}"))]
    NotLoadable {
        /// The path from which the loading was attempted.
        path: PathBuf,

        /// The source error causing the failure.
        source: std::io::Error,
    },

    /// Config can't be stored to a path.
    #[snafu(display("Config can't be stored to {path:?}"))]
    NotStorable {
        /// The path to which the storing was attempted.
        path: PathBuf,

        /// The source error causing the failure.
        source: std::io::Error,
    },

    /// The config folder can't be created under a path.
    #[snafu(display("Config folder can't be created under {path:?}"))]
    FolderNotCreatable {
        /// The path that was attempted to be created.
        path: PathBuf,

        /// The source error causing the failure.
        source: std::io::Error,
    },

    /// The system data home is not set.
    #[snafu(display("System config home not set"))]
    SystemConfigHomeNotSet,

    /// Config can't be read from a path.
    #[snafu(display("Config not readable from {path:?}"))]
    NotReadable {
        /// The path from which the reading was attempted.
        path: PathBuf,

        /// The toml deserialization error causing the failure.
        source: toml::de::Error,
    },

    /// Config can't be written to a path.
    #[snafu(display("Config not writeable to {path:?}"))]
    NotWriteable {
        /// The path to which the writing was attempted.
        path: PathBuf,

        /// The toml serialization error causing the failure.
        source: toml::ser::Error,
    },
}
