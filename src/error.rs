//! Errors

use std::io::Error as IoError;
use parser::ParseError;

/// A generic torrc error type
#[derive(Debug)]
pub struct TorrcError {
    /// Indicates what kind of error this is
    pub kind: TorrcErrorKind,
    /// Descriptiong of the error
    pub desc: &'static str,
    /// Error details
    pub detail: Option<String>
}

/// Error kinds
#[derive(PartialEq)]
#[derive(Debug)]
pub enum TorrcErrorKind {
    /// I/O error
    IoError,
    /// syntax error
    ParseError
}

/// Converts I/O Error to TorrcError
pub fn from_io_err(err: IoError) -> TorrcError {
    TorrcError {
        kind: TorrcErrorKind::IoError,
        desc: "I/O error occurred",
        detail: Some(format!("{}", err))
    }
}

/// Converts a ParseError to TorrcError
pub fn from_parse_err(err: ParseError) -> TorrcError {
    TorrcError {
        kind: TorrcErrorKind::ParseError,
        desc: "Syntax error",
        detail: Some(format!("{}", err))
    }
}
