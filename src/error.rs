//! Errors used by the crate

use std::{error::Error, fmt::Display};

/// The error returned by the derived implementation of [`FromStr`](std::str::FromStr)
/// when no scope corresponds to the given string
#[derive(Debug)]
pub struct ScopeParseError(pub String);

impl Display for ScopeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no such scope: '{}'", self.0)
    }
}

impl Error for ScopeParseError {}
