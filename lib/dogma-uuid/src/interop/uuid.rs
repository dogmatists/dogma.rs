// This is free and unencumbered software released into the public domain.

use crate::Uuid;

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
        Self::from_bytes(uuid.into_bytes())
    }
}

/// Converts to a compatible UUID value without altering any bits.
impl From<Uuid> for ::uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        Self::from_bytes(uuid.into_bytes())
    }
}
