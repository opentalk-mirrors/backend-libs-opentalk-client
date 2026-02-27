// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use snafu::Snafu;

/// The error returned from functions in this crate.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub struct DataError {
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl DataError {
    /// Create a [DataError] from a boxed error.
    pub fn new_from_boxed(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self { source }
    }

    /// Create a [DataError] from another error.
    pub fn new<E: std::error::Error + Send + Sync + 'static>(source: E) -> Self {
        Self::new_from_boxed(Box::new(source))
    }
}
