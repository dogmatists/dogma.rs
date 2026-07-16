// This is free and unencumbered software released into the public domain.

#![allow(unused)]

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
