// This is free and unencumbered software released into the public domain.

/// A trait for collections that may be countable.
pub trait MaybeCountable {
    /// Returns the number of elements in the collection, if known.
    fn maybe_count(&self) -> Option<usize> {
        None // the default
    }

    /// Checks whether the collection is countable.
    fn is_countable(&self) -> bool {
        self.maybe_count().is_some()
    }

    /// Checks whether the collection is empty, if known.
    fn maybe_is_empty(&self) -> Option<bool> {
        self.maybe_count().map(|count| count == 0)
    }

    /// Checks whether the collection is nonempty, if known.
    fn maybe_is_nonempty(&self) -> Option<bool> {
        self.maybe_is_empty().map(|result| !result)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn MaybeCountable + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.maybe_count().serialize(serializer)
    }
}
