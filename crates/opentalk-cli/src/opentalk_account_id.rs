// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::str::FromStr;

use serde::Serialize;
use snafu::{ensure, Snafu};

pub const OPENTALK_ACCOUNT_ID_MIN_LENGTH: usize = 1;

pub const OPENTALK_ACCOUNT_ID_MAX_LENGTH: usize = 100;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    derive_more::AsRef,
    derive_more::Deref,
    serde_with::DeserializeFromStr,
)]
pub struct OpenTalkAccountId(String);

/// The error that is returned by [OpenTalkAccountId::from_str] on failure.

#[derive(Debug, Snafu)]

pub enum ParseOpenTalkAccountIdError {
    /// Invalid characters were found in the input data.
    #[snafu(display("OpenTalk account id may only contain alphanumeric characters and \"-\""))]
    InvalidCharacters,

    /// The input string was shorter than the minimum length [OPENTALK_ACCOUNT_ID_MIN_LENGTH].
    #[snafu(display("OpenTalk account id must have at least {min_length} characters"))]
    TooShort {
        /// The minimum allowed length.
        min_length: usize,
    },

    /// The input string was longer than the maximum length [OPENTALK_ACCOUNT_ID_MAX_LENGTH].
    #[snafu(display("OpenTalk account id must not be longer than {max_length} characters"))]
    TooLong {
        /// The maximum allowed length.
        max_length: usize,
    },
}

impl FromStr for OpenTalkAccountId {
    type Err = ParseOpenTalkAccountIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure_is_valid(s)?;

        Ok(Self(s.to_string()))
    }
}

fn ensure_is_valid(s: &str) -> Result<(), ParseOpenTalkAccountIdError> {
    ensure!(
        s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
        InvalidCharactersSnafu
    );

    ensure!(
        s.len() >= OPENTALK_ACCOUNT_ID_MIN_LENGTH,
        TooShortSnafu {
            min_length: OPENTALK_ACCOUNT_ID_MIN_LENGTH
        }
    );

    ensure!(
        s.len() <= OPENTALK_ACCOUNT_ID_MAX_LENGTH,
        TooLongSnafu {
            max_length: OPENTALK_ACCOUNT_ID_MAX_LENGTH
        }
    );

    Ok(())
}
