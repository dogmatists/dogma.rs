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
