// This is free and unencumbered software released into the public domain.

use crate::prelude::Cow;

/// A trait for objects that may have a name.
pub trait MaybeNamed {
    /// Returns the name, if any, of the object.
    fn name(&self) -> Option<Cow<str>> {
        None // the default
    }

    /// Checks whether the object has a name.
    fn is_named(&self) -> bool {
        self.name().is_some()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn MaybeNamed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.name() {
            Some(ref value) => serializer.serialize_some(value.as_ref()),
            None => serializer.serialize_none(),
        }
    }
}
