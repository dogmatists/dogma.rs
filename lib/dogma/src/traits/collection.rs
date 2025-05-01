// This is free and unencumbered software released into the public domain.

use crate::prelude::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, Vec, VecDeque};

#[cfg(feature = "std")]
use crate::prelude::{Hash, HashMap, HashSet};

/// A trait for collections of items.
pub trait Collection {
    type Item;

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;

    /// Checks whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks whether the collection is nonempty.
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
impl<T: Ord> Collection for BinaryHeap<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `BTreeSet<T>`
impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `BTreeMap<K, V>`
impl<K: Ord, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `HashSet<T>`
#[cfg(feature = "std")]
impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// Implementation for `HashMap<K, V>`
#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
