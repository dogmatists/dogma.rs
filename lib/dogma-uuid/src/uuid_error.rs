// This is free and unencumbered software released into the public domain.

use core::fmt;

/// An error parsing a [`Uuid`](crate::Uuid) from text or a byte slice.
///
/// Display describes the problem, such as an invalid character and its
/// position, or a slice that is not exactly 16 bytes long.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UuidError(::uuid::Error);

impl UuidError {
    /// Wraps an upstream error without exposing its type.
    pub(crate) fn new(error: ::uuid::Error) -> Self {
        Self(error)
    }
}

impl core::error::Error for UuidError {}

impl fmt::Display for UuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
