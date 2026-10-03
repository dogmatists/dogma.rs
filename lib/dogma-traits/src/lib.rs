// This is free and unencumbered software released into the public domain.

//! General-purpose naming, labeling, counting, and collection traits.
//!
//! Each trait family has its own feature: `named`, `labeled`, `countable`, or
//! `collection`. All are enabled by default, along with `serde` and `std`.
//! Counting traits need no allocation; other families use `alloc`.
//! Hash-map and hash-set implementations additionally require `std`.

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "collection")]
mod collection;
#[cfg(feature = "collection")]
pub use collection::*;

#[cfg(feature = "collection")]
mod collection_mut;
#[cfg(feature = "collection")]
pub use collection_mut::*;

#[cfg(feature = "countable")]
mod countable;
#[cfg(feature = "countable")]
pub use countable::*;

#[cfg(feature = "labeled")]
mod labeled;
#[cfg(feature = "labeled")]
pub use labeled::*;

#[cfg(feature = "countable")]
mod maybe_countable;
#[cfg(feature = "countable")]
pub use maybe_countable::*;

#[cfg(feature = "labeled")]
mod maybe_labeled;
#[cfg(feature = "labeled")]
pub use maybe_labeled::*;

#[cfg(feature = "named")]
mod maybe_named;
#[cfg(feature = "named")]
pub use maybe_named::*;

#[cfg(feature = "named")]
mod named;
#[cfg(feature = "named")]
pub use named::*;
