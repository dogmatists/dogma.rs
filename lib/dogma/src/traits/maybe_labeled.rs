// This is free and unencumbered software released into the public domain.

use crate::prelude::Cow;

/// A trait for objects that may have a human-readable label.
pub trait MaybeLabeled {
    /// Returns the human-readable label, if any, of the object.
    fn label(&self) -> Option<Cow<str>> {
        None // the default
    }

    /// Checks whether the object has a human-readable label.
    fn is_labeled(&self) -> bool {
        self.label().is_some()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn MaybeLabeled {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.label() {
            Some(ref value) => serializer.serialize_some(value.as_ref()),
            None => serializer.serialize_none(),
        }
    }
}
