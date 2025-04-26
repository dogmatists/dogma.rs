// This is free and unencumbered software released into the public domain.

#![allow(unused)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

pub use alloc::{
    borrow::{self, Cow, ToOwned},
    collections::{self, BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt,
    hash::{self, Hash, Hasher},
    str::{self, FromStr},
    string::{self, String, ToString},
    vec::{self, IntoIter, Vec},
};

pub use core::result::Result;
