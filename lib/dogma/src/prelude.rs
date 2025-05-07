// This is free and unencumbered software released into the public domain.

#![allow(unused)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

pub use alloc::{
    borrow::{self, Borrow, BorrowMut, Cow, ToOwned},
    boxed::{self, Box},
    collections::{self, BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    fmt, format,
    str::{self, FromStr},
    string::{self, String, ToString},
    vec::{self, IntoIter, Vec},
};

pub use core::{
    hash::{self, Hash, Hasher},
    result::{self, Result},
};

#[cfg(feature = "std")]
pub use std::collections::{HashMap, HashSet};
