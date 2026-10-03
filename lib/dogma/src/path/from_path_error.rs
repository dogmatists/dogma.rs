// This is free and unencumbered software released into the public domain.

/// An error when parsing or converting a path into an
/// [`AncestorPath`](super::AncestorPath).
///
/// Returned by its [`FromStr`](core::str::FromStr) implementation.
#[cfg_attr(
    feature = "std",
    doc = "\nAlso returned when converting a [`std::path::Path`] into an ancestor path."
)]
#[cfg_attr(
    feature = "camino",
    doc = "\nAlso returned when converting a [`camino::Utf8Path`] into an ancestor path."
)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FromPathError {
    /// The path is empty.
    Empty,

    /// The path is not a valid ancestor path.
    NotAncestor,
}

impl core::fmt::Display for FromPathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "path is empty"),
            Self::NotAncestor => write!(f, "path is not a valid ancestor path"),
        }
    }
}

impl core::error::Error for FromPathError {}

impl FromPathError {
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidFilename`](std::io::ErrorKind::InvalidFilename)
    /// or [`InvalidData`](std::io::ErrorKind::InvalidData).
    #[cfg(feature = "std")]
    pub fn into_io_error(self) -> std::io::Error {
        use std::io::{Error, ErrorKind};
        use FromPathError::*;
        Error::new(
            match self {
                Empty => ErrorKind::InvalidData,
                NotAncestor => ErrorKind::InvalidFilename,
            },
            self,
        )
    }
}
