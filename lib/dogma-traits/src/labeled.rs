// This is free and unencumbered software released into the public domain.

use alloc::borrow::Cow;

/// A trait for objects that have a human-readable label.
pub trait Labeled {
    /// Returns the human-readable label of the object.
    fn label(&self) -> Cow<'_, str>;
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Labeled + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.label().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl_serialize_for_auto_trait_objects!(Labeled);
