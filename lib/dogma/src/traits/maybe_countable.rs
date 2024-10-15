// This is free and unencumbered software released into the public domain.

/// A trait for collections that may be countable.
pub trait MaybeCountable {
    /// Returns the number of elements in the collection, if known.
    fn count(&self) -> Option<usize> {
        None // the default
    }

    /// Checks whether the collection is countable.
    fn is_countable(&self) -> bool {
        self.count().is_some()
    }

    /// Checks whether the collection is empty, if known.
    fn is_empty(&self) -> Option<bool> {
        self.count().map(|count| count == 0)
    }

    /// Checks whether the collection is nonempty, if known.
    fn is_nonempty(&self) -> Option<bool> {
        self.is_empty().map(|result| !result)
    }
}
