// This is free and unencumbered software released into the public domain.

use core::fmt;

/// A 128-bit universally unique identifier.
///
/// UUIDs are stored by value and are cheap to copy. All 128-bit patterns are
/// accepted: construction does not validate a version or variant, generate
/// randomness, or guarantee uniqueness. The default value is the all-zero
/// (nil) UUID.
///
/// Byte conversions use network byte order, matching the left-to-right order
/// of hexadecimal pairs in the canonical `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
/// representation. They do not use the mixed-endian layout of some GUID APIs.
/// Equality, ordering, and hashing use the identifier's value; ordering is
/// lexicographic over those bytes. Display produces lowercase, hyphenated text.
///
/// With `serde`, human-readable formats use the canonical string. Binary
/// formats use a 16-byte byte string, without a wrapper tag. Deserialization
/// accepts UUID text in human-readable formats and exactly 16 bytes in binary
/// formats.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uuid(::uuid::Uuid);

impl Uuid {
    /// Constructs an identifier from 16 bytes in network byte order.
    ///
    /// Every bit is preserved, without validating or changing the version or
    /// variant fields.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(::uuid::Uuid::from_bytes(bytes))
    }

    /// Borrows the identifier's 16 bytes in network byte order.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    /// Returns the identifier's 16 bytes in network byte order.
    pub const fn into_bytes(self) -> [u8; 16] {
        self.0.into_bytes()
    }
}

/// Constructs an identifier without altering its bytes.
impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Self::from_bytes(bytes)
    }
}

/// Extracts the identifier's bytes in network byte order.
impl From<Uuid> for [u8; 16] {
    fn from(uuid: Uuid) -> Self {
        uuid.into_bytes()
    }
}

/// Copies the identifier's 16 bytes into a new vector in network byte order.
///
/// Requires the `alloc` feature (enabled by default and by `std`).
///
/// ```
/// extern crate alloc;
///
/// let id = dogma_uuid::Uuid::from_bytes([0xab; 16]);
/// let bytes: alloc::vec::Vec<u8> = id.into();
/// assert_eq!(bytes.as_slice(), &[0xab; 16]);
/// ```
#[cfg(feature = "alloc")]
impl From<Uuid> for alloc::vec::Vec<u8> {
    fn from(uuid: Uuid) -> Self {
        Self::from(uuid.into_bytes())
    }
}

/// Converts a compatible UUID value without altering any bits.
///
/// ```
/// let original = uuid::Uuid::from_bytes([0xab; 16]);
/// let id = dogma_uuid::Uuid::from(original);
/// let round_trip: uuid::Uuid = id.into();
/// assert_eq!(round_trip, original);
/// ```
impl From<::uuid::Uuid> for Uuid {
    fn from(uuid: ::uuid::Uuid) -> Self {
        Self(uuid)
    }
}

/// Converts to a compatible UUID value without altering any bits.
impl From<Uuid> for ::uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        uuid.0
    }
}

impl AsRef<[u8; 16]> for Uuid {
    fn as_ref(&self) -> &[u8; 16] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for Uuid {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uuid {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.0, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <::uuid::Uuid as serde::Deserialize>::deserialize(deserializer).map(Self)
    }
}
