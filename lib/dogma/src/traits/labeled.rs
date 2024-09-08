// This is free and unencumbered software released into the public domain.

use crate::prelude::Cow;

/// A trait for objects that have a human-readable label.
pub trait Labeled {
    /// Returns the human-readable label of the object.
    fn label(&self) -> Cow<str>;
}
