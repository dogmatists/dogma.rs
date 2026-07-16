// This is free and unencumbered software released into the public domain.

/// A possible error when converting a [`Path`] or [`Utf8Path`] into an
/// [`AncestorPath`].
///
/// Produced by the [`TryFrom<Path>`][tryfrom1] and [`TryFrom<Utf8Path>`][tryfrom2]
/// implementations for [`AncestorPath`](AncestorPath).
///
/// [tryfrom1]: AncestorPath#impl-TryFrom<Path>-for-AncestorPath
/// [tryfrom2]: AncestorPath#impl-TryFrom<Utf8Path>-for-AncestorPath
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
