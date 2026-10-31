// This is free and unencumbered software released into the public domain.

use crate::{UuidError, UuidVariant, UuidVersion};
use core::{fmt, str::FromStr};

/// A 128-bit universally unique identifier.
///
/// UUIDs are stored by value and are cheap to copy. All 128-bit patterns are
/// accepted: byte conversion does not validate a version or variant, generate
/// randomness, or guarantee uniqueness. The default value is the all-zero
/// (nil) UUID.
///
/// Byte conversions use network byte order, matching the left-to-right order
/// of hexadecimal pairs in the canonical `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
/// representation. They do not use the mixed-endian layout of some GUID APIs.
/// Equality, ordering, and hashing use the identifier's value; ordering is
/// lexicographic over those bytes. Display and [`fmt::LowerHex`] produce
/// lowercase hyphenated text; [`fmt::UpperHex`] produces uppercase. Parsing
/// with [`FromStr`] accepts the simple, hyphenated, braced, and `urn:uuid:`
/// forms in either case.
///
/// With `serde`, human-readable formats use the canonical string. Binary
/// formats use a 16-byte byte string, without a wrapper tag. Deserialization
/// accepts UUID text in human-readable formats and exactly 16 bytes in binary
/// formats.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uuid(::uuid::Uuid);

impl Uuid {
    /// The nil UUID, with all 128 bits set to zero. Also the default value.
    pub const NIL: Self = Self(::uuid::Uuid::nil());

    /// The max UUID, with all 128 bits set to one.
    pub const MAX: Self = Self(::uuid::Uuid::max());
    /// Generates a random version 4 UUID.
    ///
    /// Requires `v4` and a supported random source. Enable `js` when targeting
    /// `wasm32-unknown-unknown`.
    ///
    /// # Panics
    ///
    /// Panics if the random source fails.
    #[cfg(feature = "v4")]
    #[must_use]
    pub fn new_v4() -> Self {
        Self(::uuid::Uuid::new_v4())
    }

    /// Generates a time-ordered version 7 UUID using the current Unix time.
    ///
    /// Requires `v7`, `std`, and a supported random source. Enable `js` when
    /// targeting `wasm32-unknown-unknown`.
    ///
    /// # Panics
    ///
    /// Panics if the random source fails.
    #[cfg(all(feature = "v7", feature = "std"))]
    #[must_use]
    pub fn now_v7() -> Self {
        Self(::uuid::Uuid::now_v7())
    }

    /// Constructs an identifier from 16 bytes in network byte order.
    ///
    /// Every bit is preserved, without validating or changing the version or
    /// variant fields.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(::uuid::Uuid::from_bytes(bytes))
    }

    /// Borrows the identifier's 16 bytes in network byte order.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    /// Returns the identifier's 16 bytes in network byte order.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 16] {
        self.0.into_bytes()
    }

    /// Constructs an identifier from a 128-bit integer, most significant byte
    /// first, preserving every bit.
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(::uuid::Uuid::from_u128(value))
    }

    /// Returns the identifier as a 128-bit integer, most significant byte first.
    #[must_use]
    pub const fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    /// Checks whether all 128 bits are zero.
    #[must_use]
    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    /// Checks whether all 128 bits are one.
    #[must_use]
    pub const fn is_max(&self) -> bool {
        self.0.is_max()
    }

    /// Returns the recognized version, or `None` for unassigned version numbers.
    ///
    /// The nil and max UUIDs report [`UuidVersion::Nil`] and
    /// [`UuidVersion::Max`]. Use [`Self::version_num`] for the raw field.
    #[must_use]
    pub const fn version(&self) -> Option<UuidVersion> {
        Some(match self.version_num() {
            0 if self.is_nil() => UuidVersion::Nil,
            1 => UuidVersion::TimeBased,
            2 => UuidVersion::DceSecurity,
            3 => UuidVersion::Md5,
            4 => UuidVersion::Random,
            5 => UuidVersion::Sha1,
            6 => UuidVersion::ReorderedTimeBased,
            7 => UuidVersion::UnixTimeBased,
            8 => UuidVersion::Custom,
            15 if self.is_max() => UuidVersion::Max,
            _ => return None,
        })
    }

    /// Returns the 4-bit version field as a number in `0..=15`.
    ///
    /// The field is read as is, regardless of the variant.
    #[must_use]
    pub const fn version_num(&self) -> u8 {
        self.as_bytes()[6] >> 4
    }

    /// Returns the variant, which determines the layout of the other fields.
    #[must_use]
    pub const fn variant(&self) -> UuidVariant {
        match self.as_bytes()[8] {
            byte if byte & 0x80 == 0x00 => UuidVariant::Ncs,
            byte if byte & 0xc0 == 0x80 => UuidVariant::Rfc9562,
            byte if byte & 0xe0 == 0xc0 => UuidVariant::Microsoft,
            _ => UuidVariant::Future,
        }
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

/// Constructs an identifier from a 128-bit integer without altering any bits.
impl From<u128> for Uuid {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

/// Extracts the identifier as a 128-bit integer.
impl From<Uuid> for u128 {
    fn from(uuid: Uuid) -> Self {
        uuid.as_u128()
    }
}

/// Copies exactly 16 bytes in network byte order, failing for other lengths.
impl TryFrom<&[u8]> for Uuid {
    type Error = UuidError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        ::uuid::Uuid::from_slice(bytes)
            .map(Self)
            .map_err(UuidError::new)
    }
}

/// Parses the simple, hyphenated, braced, or `urn:uuid:` form in either case.
///
/// ```
/// use dogma_uuid::Uuid;
///
/// let id: Uuid = "{67E55044-10B1-426F-9247-BB680E5FE0C8}".parse().unwrap();
/// assert_eq!(id.to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
/// assert!("67e55044".parse::<Uuid>().is_err());
/// ```
impl FromStr for Uuid {
    type Err = UuidError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ::uuid::Uuid::try_parse(text)
            .map(Self)
            .map_err(UuidError::new)
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

impl fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
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
