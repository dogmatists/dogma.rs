// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use dogma::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub mod prelude;

#[cfg(feature = "enums")]
pub mod enums;

mod features;
pub use features::*;

/// Common traits for objects.
#[cfg(any(
    feature = "traits",
    any(
        feature = "collection",
        feature = "countable",
        feature = "labeled",
        feature = "named"
    )
))]
pub mod traits;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
