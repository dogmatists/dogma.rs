// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use dogma::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub mod prelude;

mod features;
pub use features::*;

/// Common traits for objects.
pub mod traits;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
