// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Data persistency functions for the OpenTalk client library

#![deny(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    overflowing_literals,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod account_data_file;
mod account_tokens;
mod data_error;
mod data_manager;
mod opentalk_account_id;
mod opentalk_instance_account_id;
mod opentalk_instance_id;

pub use account_tokens::AccountTokens;
pub use data_error::DataError;
pub use data_manager::DataManager;
pub use opentalk_account_id::OpenTalkAccountId;
pub use opentalk_instance_account_id::OpenTalkInstanceAccountId;
pub use opentalk_instance_id::OpenTalkInstanceId;
