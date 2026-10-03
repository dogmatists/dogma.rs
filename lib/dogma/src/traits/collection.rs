// This is free and unencumbered software released into the public domain.

use crate::prelude::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, Vec, VecDeque};

#[cfg(feature = "std")]
use crate::prelude::{HashMap, HashSet};

/// Reports a collection's item count and whether it is empty.
///
/// Implementors must keep [`len`](Self::len), [`is_empty`](Self::is_empty), and
/// [`is_nonempty`](Self::is_nonempty) consistent: a collection is empty exactly
/// when its length is zero. This trait does not provide iteration or item access.
///
/// Available with the `collection` feature. Implementations for hash maps and
/// hash sets additionally require `std`.
pub trait Collection {
    /// The type of an item in the collection.
    ///
    /// For maps this is the key-value pair `(K, V)`, so each entry counts as one
    /// item. For sequences and sets this is the element type `T`.
    type Item;

    /// Returns the number of items currently in the collection, not its capacity.
    ///
    /// Arrays always report their fixed length; slices report their slice length.
    fn len(&self) -> usize;

    /// Returns `true` exactly when [`len`](Self::len) is zero.
    ///
    /// Overrides must preserve this equivalence.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the logical negation of [`is_empty`](Self::is_empty).
    fn is_nonempty(&self) -> bool {
        !self.is_empty()
    }
}

// Implementation for fixed-size arrays
impl<T, const N: usize> Collection for [T; N] {
    type Item = T;

    fn len(&self) -> usize {
        N
    }

    fn is_empty(&self) -> bool {
        N == 0
    }
}

// Implementation for slices
impl<T> Collection for [T] {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `Vec<T>`
impl<T> Collection for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `VecDeque<T>`
impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `LinkedList<T>`
impl<T> Collection for LinkedList<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `BinaryHeap<T>`
impl<T> Collection for BinaryHeap<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `BTreeSet<T>`
impl<T> Collection for BTreeSet<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `BTreeMap<K, V>`
impl<K, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `HashSet<T, S>`
#[cfg(feature = "std")]
impl<T, S> Collection for HashSet<T, S> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `HashMap<K, V, S>`
#[cfg(feature = "std")]
impl<K, V, S> Collection for HashMap<K, V, S> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
