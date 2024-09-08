// This is free and unencumbered software released into the public domain.

use crate::prelude::Cow;

/// A trait for objects that may have a name.
pub trait MaybeNamed {
    /// Returns the name, if any, of the object.
    fn name(&self) -> Option<Cow<str>>;

    /// Checks whether the object has a name.
    fn is_named(&self) -> bool {
        self.name().is_some()
    }
}
