// This is free and unencumbered software released into the public domain.

/// An error when parsing or converting a path into an
/// [`AncestorPath`](super::AncestorPath).
///
/// Returned by its [`FromStr`](core::str::FromStr) implementation.
/// Validation is lexical: neither variant indicates a filesystem lookup failure.
/// The error retains no input path and has no underlying error source.
/// Available with the `alloc` feature.
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
    /// No parent-directory components remain after ignoring `.` components.
    ///
    /// Includes empty input and current-directory-only paths such as `./.`.
    Empty,

    /// The path contains a root, drive prefix, or named component.
    ///
    /// Named components are not canceled by later parents: `child/..` is
    /// rejected. Strings accept both slash styles; native path conversions
    /// follow the platform's component rules.
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
    /// Wraps this error in a [`std::io::Error`], retaining it as the payload.
    ///
    /// Maps [`Self::Empty`] to [`InvalidData`](std::io::ErrorKind::InvalidData)
    /// and [`Self::NotAncestor`] to
    /// [`InvalidFilename`](std::io::ErrorKind::InvalidFilename).
    /// Requires `std` and performs no filesystem access.
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
