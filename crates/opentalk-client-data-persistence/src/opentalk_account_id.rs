// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::str::FromStr;

use serde::Serialize;
use snafu::{ensure, Snafu};

pub const OPENTALK_ACCOUNT_ID_MIN_LENGTH: usize = 1;

pub const OPENTALK_ACCOUNT_ID_MAX_LENGTH: usize = 100;

/// The id of an OpenTalk account.
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_matches;

    use super::OpenTalkAccountId;
    use crate::opentalk_account_id::ParseOpenTalkAccountIdError;

    #[test]
    fn success_from_str() {
        let id: Result<OpenTalkAccountId, ParseOpenTalkAccountIdError> = "opentalk-id".parse();
        assert!(id.is_ok());
    }

    #[test]
    fn error_to_short_from_str() {
        let id: Result<OpenTalkAccountId, ParseOpenTalkAccountIdError> = "".parse();
        assert_matches!(
            id,
            Err(ParseOpenTalkAccountIdError::TooShort { min_length: _ })
        );
    }

    #[test]
    fn error_to_long_from_str() {
        let id: Result<OpenTalkAccountId, ParseOpenTalkAccountIdError> = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".parse();
        assert_matches!(
            id,
            Err(ParseOpenTalkAccountIdError::TooLong { max_length: _ })
        );
    }

    #[test]
    fn error_invalid_char_from_str() {
        let id: Result<OpenTalkAccountId, ParseOpenTalkAccountIdError> = "*".parse();
        assert_matches!(id, Err(ParseOpenTalkAccountIdError::InvalidCharacters));
    }
}
