// This is free and unencumbered software released into the public domain.

use super::Collection;
use crate::prelude::{
    BTreeMap, BTreeSet, BinaryHeap, Hash, HashMap, HashSet, LinkedList, Vec, VecDeque,
};

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

// Implementation for `HashSet<T>`
impl<T: Eq + Hash> CollectionMut for HashSet<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

// Implementation for `HashMap<K, V>`
impl<K: Eq + Hash, V> CollectionMut for HashMap<K, V> {
    fn clear(&mut self) {
        self.clear()
    }
}
