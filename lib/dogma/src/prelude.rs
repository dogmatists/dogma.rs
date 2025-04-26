// This is free and unencumbered software released into the public domain.

#![allow(unused)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

pub use alloc::{
    borrow::{Cow, ToOwned},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::{Hash, Hasher},
    string::{String, ToString},
    vec::{IntoIter, Vec},
};

pub use core::result::Result;
