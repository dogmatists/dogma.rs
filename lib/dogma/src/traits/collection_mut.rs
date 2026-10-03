// This is free and unencumbered software released into the public domain.

use super::Collection;
use crate::prelude::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, Vec, VecDeque};

#[cfg(feature = "std")]
use crate::prelude::{HashMap, HashSet};

/// A collection whose items can all be removed.
///
/// Extends [`Collection`] with [`clear`](Self::clear). Arrays and slices implement
/// `Collection` but not this trait, because their length cannot be changed through
/// a mutable reference.
///
/// Available with the `collection` feature. Implementations for hash maps and
/// hash sets additionally require `std`.
pub trait CollectionMut: Collection {
    /// Removes all items from the collection.
    ///
    /// After this method returns normally, [`len`](Collection::len) is zero and
    /// [`is_empty`](Collection::is_empty) is `true`. Clearing an empty collection
    /// leaves it empty.
    ///
    /// The provided implementations delegate to the collection's inherent
    /// `clear` method, dropping its owned items. Retaining or releasing allocated
    /// capacity is collection-specific; this trait guarantees neither.
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
impl<T> CollectionMut for BinaryHeap<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `BTreeSet<T>`
impl<T> CollectionMut for BTreeSet<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `BTreeMap<K, V>`
impl<K, V> CollectionMut for BTreeMap<K, V> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `HashSet<T, S>`
#[cfg(feature = "std")]
impl<T, S> CollectionMut for HashSet<T, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `HashMap<K, V, S>`
#[cfg(feature = "std")]
impl<K, V, S> CollectionMut for HashMap<K, V, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "std")]
    use core::hash::BuildHasherDefault;
    #[cfg(feature = "std")]
    use std::hash::DefaultHasher;

    #[test]
    fn collection_traits_accept_unconstrained_items() {
        struct Item;

        fn assert_traits<C: CollectionMut>() {}

        assert_traits::<BinaryHeap<Item>>();
        assert_traits::<BTreeSet<Item>>();
        assert_traits::<BTreeMap<Item, Item>>();

        #[cfg(feature = "std")]
        {
            assert_traits::<HashSet<Item, ()>>();
            assert_traits::<HashMap<Item, Item, ()>>();
        }
    }

    #[cfg(feature = "std")]
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
    #[cfg(feature = "std")]
    fn hash_map_with_custom_hasher() {
        let mut map = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
        map.insert("first", 1);
        map.insert("second", 2);
        check_clear(map);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hash_set_with_custom_hasher() {
        let mut set = HashSet::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
        set.insert("first");
        set.insert("second");
        check_clear(set);
    }
}
