// This is free and unencumbered software released into the public domain.

/// A trait for collections that are countable.
pub trait Countable {
    /// Returns the number of elements in the collection.
    fn count(&self) -> usize; // FIXME: change to `len()`?

    /// Checks whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Checks whether the collection is nonempty.
    fn is_nonempty(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Countable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.count().serialize(serializer)
    }
}
