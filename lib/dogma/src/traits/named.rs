// This is free and unencumbered software released into the public domain.

use crate::prelude::Cow;

/// A trait for objects that have a name.
pub trait Named {
    /// Returns the name of the object.
    fn name(&self) -> Cow<str>;
}
