// This is free and unencumbered software released into the public domain.

/// A recognized UUID version, identifying how an identifier was generated.
///
/// Returned by [`Uuid::version`](crate::Uuid::version). Each discriminant is
/// the value of the 4-bit version field, so `version as u8` yields it. The nil
/// and max UUIDs have their own entries; other identifiers with version field
/// 0 or 15 have no recognized version. See
/// [RFC 9562, section 4.2](https://www.rfc-editor.org/rfc/rfc9562#section-4.2).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(u8)]
pub enum UuidVersion {
    /// The nil UUID, with all bits set to zero.
    Nil = 0,
    /// Version 1: a Gregorian timestamp and a node identifier.
    TimeBased = 1,
    /// Version 2: reserved for DCE Security.
    DceSecurity = 2,
    /// Version 3: an MD5 hash of a namespace and a name.
    Md5 = 3,
    /// Version 4: random or pseudorandom bits.
    Random = 4,
    /// Version 5: a SHA-1 hash of a namespace and a name.
    Sha1 = 5,
    /// Version 6: a version 1 timestamp reordered for sorting.
    ReorderedTimeBased = 6,
    /// Version 7: a Unix timestamp in milliseconds and random bits.
    UnixTimeBased = 7,
    /// Version 8: a custom, application-specific layout.
    Custom = 8,
    /// The max UUID, with all bits set to one.
    Max = 15,
}
