// Copyright 2022 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

//! Bulletproofs+ error definitions

use thiserror_no_std::Error;

/// Represents an error in proof creation, verification, or parsing.
#[derive(Debug, Error)]
pub enum ProofError {
    /// A proof component failed to verify
    #[error("A proof component failed to verify: `{0}")]
    VerificationFailed(String),
    /// Internal data sizes are inconsistent
    #[error("Internal data is invalid: `{0}`")]
    InvalidArgument(String),
    /// Invalid array/vector length error
    #[error("Invalid array/vector length error: `{0}`")]
    InvalidLength(String),
    /// Invalid `Blake2b` hash operation
    #[error("Invalid Blake2b")]
    InvalidBlake2b,
    /// Internal size overflow
    #[error("Internal size overflow")]
    SizeOverflow,
}
