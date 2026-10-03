// This is free and unencumbered software released into the public domain.

use super::Collection;
use crate::prelude::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, Vec, VecDeque};

#[cfg(feature = "std")]
use crate::prelude::{Hash, HashMap, HashSet};

/// A trait for collections of items.
pub trait CollectionMut: Collection {
    fn clear(&mut self);
}

// Implementation for `Vec<T>`
impl<T> CollectionMut for Vec<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `VecDeque<T>`
impl<T> CollectionMut for VecDeque<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `LinkedList<T>`
impl<T> CollectionMut for LinkedList<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `BinaryHeap<T>`
impl<T: Ord> CollectionMut for BinaryHeap<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `BTreeSet<T>`
impl<T: Ord> CollectionMut for BTreeSet<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `BTreeMap<K, V>`
impl<K: Ord, V> CollectionMut for BTreeMap<K, V> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `HashSet<T, S>`
#[cfg(feature = "std")]
impl<T: Eq + Hash, S> CollectionMut for HashSet<T, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `HashMap<K, V, S>`
#[cfg(feature = "std")]
impl<K: Eq + Hash, V, S> CollectionMut for HashMap<K, V, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use core::hash::BuildHasherDefault;
    use std::hash::DefaultHasher;

    fn check_clear<C: CollectionMut>(mut collection: C) {
        assert_eq!(collection.len(), 2);
        assert!(!collection.is_empty());
        assert!(collection.is_nonempty());

        collection.clear();

        assert_eq!(collection.len(), 0);
        assert!(collection.is_empty());
        assert!(!collection.is_nonempty());
    }

    #[test]
    fn hash_map_with_custom_hasher() {
        let mut map = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
        map.insert("first", 1);
        map.insert("second", 2);
        check_clear(map);
    }

    #[test]
    fn hash_set_with_custom_hasher() {
        let mut set = HashSet::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
        set.insert("first");
        set.insert("second");
        check_clear(set);
    }
}
