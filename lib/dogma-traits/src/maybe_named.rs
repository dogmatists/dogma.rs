// This is free and unencumbered software released into the public domain.

use alloc::borrow::Cow;

/// A trait for objects that may have a name.
pub trait MaybeNamed {
    /// Returns the name, if any, of the object.
    fn maybe_name(&self) -> Option<Cow<'_, str>> {
        None // the default
    }

    /// Checks whether the object has a name.
    fn is_named(&self) -> bool {
        self.maybe_name().is_some()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn MaybeNamed + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.maybe_name() {
            Some(ref value) => serializer.serialize_some(value.as_ref()),
            None => serializer.serialize_none(),
        }
    }
}
