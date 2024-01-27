/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::error::Error;
use std::fmt;
use std::io;
use std::num;
use std::string;

#[derive(Debug)]
pub struct elosysError {
    pub kind: elosysErrorKind,
    pub source: Option<Box<dyn Error>>,
    pub backtrace: Backtrace,
}

/// Error type to handle all errors within the code and dependency-raised
/// errors. This serves 2 purposes. The first is to keep a consistent error type
/// in the code to reduce the cognitive load needed for using Result and Error
/// types. The second is to give a singular type to convert into NAPI errors to
/// be raised on the Javascript side.
#[derive(Debug, PartialEq)]
pub enum elosysErrorKind {
    BellpersonSynthesis,
    CryptoBox,
    FrostLibError,
    IllegalValue,
    InconsistentWitness,
    InvalidAssetIdentifier,
    InvalidAuthorizingKey,
    InvalidBalance,
    InvalidCommitment,
    InvalidData,
    InvalidDecryptionKey,
    InvalidDiversificationPoint,
    InvalidEntropy,
    InvalidLanguageEncoding,
    InvalidMinersFeeTransaction,
    InvalidMintProof,
    InvalidMintSignature,
    InvalidMnemonicString,
    InvalidNonceLength,
    InvalidNullifierDerivingKey,
    InvalidOutputProof,
    InvalidPaymentAddress,
    InvalidPublicAddress,
    InvalidSecret,
    InvalidSignature,
    InvalidSigningKey,
    InvalidSpendProof,
    InvalidSpendSignature,
    InvalidTransaction,
    InvalidTransactionVersion,
    InvalidViewingKey,
    InvalidWord,
    Io,
    IsSmallOrder,
    RandomnessError,
    RoundTwoSigningFailure,
    TryFromInt,
    Utf8,
}

impl elosysError {
    pub fn new(kind: elosysErrorKind) -> Self {
        Self {
            kind,
            source: None,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn new_with_source<E>(kind: elosysErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn Error>>,
    {
        Self {
            kind,
            source: Some(source.into()),
            backtrace: Backtrace::capture(),
        }
    }
}

impl Error for elosysError {}

impl fmt::Display for elosysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let has_backtrace = self.backtrace.status() == BacktraceStatus::Captured;
        write!(f, "{:?}", self.kind)?;
        if let Some(source) = &self.source {
            write!(f, "\nCaused by: \n{}", source)?;
        }
        if has_backtrace {
            write!(f, "\nBacktrace:\n{:2}", self.backtrace)
        } else {
            write!(f, "\nTo enable Rust backtraces, use RUST_BACKTRACE=1")
        }
    }
}

impl From<io::Error> for elosysError {
    fn from(e: io::Error) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::Io, e)
    }
}

impl From<crypto_box::aead::Error> for elosysError {
    fn from(e: crypto_box::aead::Error) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::CryptoBox, e)
    }
}

impl From<string::FromUtf8Error> for elosysError {
    fn from(e: string::FromUtf8Error) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::Utf8, e)
    }
}

impl From<bellperson::SynthesisError> for elosysError {
    fn from(e: bellperson::SynthesisError) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::BellpersonSynthesis, e)
    }
}

impl From<num::TryFromIntError> for elosysError {
    fn from(e: num::TryFromIntError) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::TryFromInt, e)
    }
}

impl From<elosys_frost::frost::Error> for elosysError {
    fn from(e: elosys_frost::frost::Error) -> elosysError {
        elosysError::new_with_source(elosysErrorKind::FrostLibError, e)
    }
}
