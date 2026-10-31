// This is free and unencumbered software released into the public domain.

/// A UUID variant, determining the layout of the identifier's other fields.
///
/// Returned by [`Uuid::variant`](crate::Uuid::variant). The variant is read
/// from the most significant bits of byte 8; these four cases cover every bit
/// pattern. See
/// [RFC 9562, section 4.1](https://www.rfc-editor.org/rfc/rfc9562#section-4.1).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UuidVariant {
    /// Reserved for NCS backward compatibility (`0xxx`), including the nil UUID.
    Ncs,
    /// The variant of RFC 9562 and its predecessor RFC 4122 (`10xx`), used by
    /// nearly all UUIDs.
    Rfc9562,
    /// Reserved for Microsoft backward compatibility (`110x`).
    Microsoft,
    /// Reserved for future definition (`111x`), including the max UUID.
    Future,
}
